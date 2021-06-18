use crate::action::{Action, ActionList, StoredAction};
use crate::common::{Penalty, Solve};
use crate::request::{SyncRequest, SyncResponse};
use crate::storage::Storage;
use crate::sync::{SyncOperation, SyncStatus};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[cfg(feature = "native-storage")]
use crate::storage::RocksDBStorage;
#[cfg(feature = "native-storage")]
use dirs::data_local_dir;
#[cfg(feature = "native-storage")]
use std::path::Path;

#[cfg(feature = "web-storage")]
use crate::storage::WebStorage;

const UNSYNCED: u32 = 0;

pub struct History {
    storage: Box<dyn Storage>,
    solves: SolveDatabase,
    synced_solves: SolveDatabase,
    synced_actions: ActionList,
    sync_key: String,
    sync_id: u32,
    local_actions: ActionList,
    current_sync: Option<Arc<Mutex<SyncOperation>>>,
    last_sync_result: SyncStatus,
    current_session: String,
    update_id: u64,
    next_update_id: u64,
}

#[derive(Clone)]
struct SolveDatabase {
    solves: HashMap<String, Solve>,
    sessions: HashMap<String, Session>,
    actions: HashSet<String>,
}

#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub name: Option<String>,
    pub solves: HashSet<String>,
    pub update_id: u64,
}

impl History {
    #[cfg(feature = "native-storage")]
    pub fn open() -> Result<Self> {
        let mut path =
            data_local_dir().ok_or_else(|| anyhow!("Local data directory not defined"))?;
        path.push("tpscube");
        path.push("solves");
        Self::open_at(path)
    }

    #[cfg(feature = "native-storage")]
    pub fn open_at<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Open up the local database and read actions from it
        Self::open_with_storage(Box::new(RocksDBStorage::open(path.as_ref())?))
    }

    #[cfg(feature = "web-storage")]
    pub fn open() -> Result<Self> {
        Self::open_with_storage(Box::new(WebStorage))
    }

    fn open_with_storage(mut storage: Box<dyn Storage>) -> Result<Self> {
        let mut synced_actions = ActionList::load(storage.as_ref(), "synced")?;
        let mut local_actions = ActionList::load(storage.as_ref(), "local")?;

        // Fetch sync information from local database
        let mut sync_key = match storage.get("sync_key")? {
            Some(key) => Some(
                SyncRequest::validate_sync_key(&String::from_utf8_lossy(&key))
                    .ok_or_else(|| anyhow!("Invalid sync key"))?,
            ),
            None => None,
        };
        let mut sync_id = match storage.get("sync_id")? {
            Some(raw_id) => Some(u32::from_le_bytes(
                raw_id.try_into().map_err(|_| anyhow!("Invalid sync ID"))?,
            )),
            None => None,
        };

        if sync_key.is_none() || sync_id.is_none() {
            // No valid sync information in the database, create new sync information
            sync_key = Some(SyncRequest::new_sync_key());
            sync_id = Some(UNSYNCED);
            storage.put("sync_key", sync_key.as_ref().unwrap().as_bytes())?;
            storage.put("sync_id", &sync_id.unwrap().to_le_bytes())?;

            // If there was synced information that is now invalid, move it to local so
            // that can be synced under the new key and data loss is avoided.
            if synced_actions.has_actions() {
                local_actions.prepend(&mut synced_actions);
                local_actions.save_index(storage.as_mut())?;
                synced_actions.save_index(storage.as_mut())?;
            }
        }

        let current_session = match storage.get("session")? {
            Some(session) => String::from_utf8_lossy(&session).into_owned(),
            None => {
                let session = Uuid::new_v4().to_simple().to_string();
                storage.put("session", session.as_bytes())?;
                session
            }
        };

        let mut result = Self {
            storage,
            solves: SolveDatabase::new(),
            synced_solves: SolveDatabase::new(),
            synced_actions,
            sync_key: sync_key.unwrap(),
            sync_id: sync_id.unwrap(),
            local_actions,
            current_sync: None,
            last_sync_result: SyncStatus::NotSynced,
            current_session,
            update_id: 0,
            next_update_id: 1,
        };

        // Resolve actions to create solve and session lists
        for action in &result.synced_actions {
            result
                .synced_solves
                .resolve_action(action, &mut result.next_update_id);
        }
        result.solves = result.synced_solves.clone();
        for action in &result.local_actions {
            result
                .solves
                .resolve_action(action, &mut result.next_update_id);
        }

        Ok(result)
    }

    pub fn solves(&self) -> &HashMap<String, Solve> {
        &self.solves.solves
    }

    pub fn sessions(&self) -> &HashMap<String, Session> {
        &self.solves.sessions
    }

    pub fn update_id(&self) -> u64 {
        self.update_id
    }

    fn new_action(&mut self, action: StoredAction) {
        if self
            .solves
            .resolve_action(&action, &mut self.next_update_id)
        {
            self.local_actions.push(action);
            self.update_id = self.next_update_id;
            self.next_update_id += 1;
        }
    }

    pub fn new_solve(&mut self, solve: Solve) {
        self.new_action(StoredAction::new(Action::NewSolve(solve)));
    }

    pub fn new_session(&mut self) -> Result<String> {
        let session = Uuid::new_v4().to_simple().to_string();
        self.current_session = session.clone();
        self.storage.put("session", session.as_bytes())?;
        self.update_id = self.next_update_id;
        self.next_update_id += 1;
        Ok(session)
    }

    pub fn current_session(&self) -> &str {
        &self.current_session
    }

    pub fn set_current_session(&mut self, session: String) {
        self.current_session = session;
        self.update_id = self.next_update_id;
        self.next_update_id += 1;
    }

    pub fn penalty(&mut self, solve_id: String, penalty: Penalty) {
        self.new_action(StoredAction::new(Action::Penalty(solve_id, penalty)));
    }

    pub fn change_session(&mut self, solve_id: String, session_id: String) {
        self.new_action(StoredAction::new(Action::ChangeSession(
            solve_id, session_id,
        )));
    }

    pub fn merge_sessions(&mut self, first_id: String, second_id: String) {
        self.new_action(StoredAction::new(Action::MergeSessions(
            first_id, second_id,
        )));
    }

    pub fn rename_session(&mut self, session_id: String, name: String) {
        self.new_action(StoredAction::new(Action::RenameSession(
            session_id,
            Some(name),
        )));
    }

    pub fn default_session_name(&mut self, session_id: String) {
        self.new_action(StoredAction::new(Action::RenameSession(session_id, None)));
    }

    pub fn delete_solve(&mut self, solve_id: String) {
        self.new_action(StoredAction::new(Action::DeleteSolve(solve_id)));
    }

    pub fn local_commit(&mut self) -> Result<()> {
        self.local_actions.commit(self.storage.as_mut(), false)
    }

    fn sync_request(&self) -> SyncRequest {
        // Gather local actions for syncing
        let actions: Vec<StoredAction> = self
            .local_actions
            .iter()
            .map(|action| action.clone())
            .collect();

        // Create the sync request with the current sync key and sync ID, along
        // with the local actions that need to be uploaded.
        SyncRequest {
            sync_key: self.sync_key.clone(),
            sync_id: self.sync_id,
            upload: if actions.len() == 0 {
                None
            } else {
                Some(actions)
            },
        }
    }

    pub fn start_sync(&mut self) -> bool {
        // Do not start another sync if one is already running
        if self.current_sync.is_none() {
            self.current_sync = Some(SyncOperation::new(self.sync_request()));
            true
        } else {
            false
        }
    }

    pub fn check_sync_status(&mut self) -> SyncStatus {
        match self.current_sync.clone() {
            Some(sync) => {
                // There is a sync active, check for completion
                let sync = sync.lock().unwrap();
                if sync.done() {
                    // Sync request is done, check response
                    match &sync.response().as_ref().unwrap() {
                        Ok(response) => {
                            // Response is OK, process it now
                            self.current_sync = None;
                            if let Err(error) = self.resolve_sync(response) {
                                self.last_sync_result = SyncStatus::SyncFailed(error.to_string());
                            }

                            // Response processing may have triggered another sync stage. Check
                            // for another pending sync, or if there isn't one, return the status
                            // of the completed sync.
                            if self.current_sync.is_some() {
                                SyncStatus::SyncPending
                            } else {
                                self.last_sync_result.clone()
                            }
                        }
                        Err(error) => {
                            // Sync failed, save failure message and return it
                            self.current_sync = None;
                            self.last_sync_result = SyncStatus::SyncFailed(error.to_string());
                            self.last_sync_result.clone()
                        }
                    }
                } else {
                    // Sync request is still pending
                    SyncStatus::SyncPending
                }
            }
            None => {
                // No sync requests currently pending, return last sync result
                self.last_sync_result.clone()
            }
        }
    }

    fn resolve_sync(&mut self, response: &SyncResponse) -> Result<()> {
        if response.new_actions.len() != 0 {
            // There are new actions, commit them to the synced state
            for action in &response.new_actions {
                self.synced_solves
                    .resolve_action(action, &mut self.next_update_id);
                self.synced_actions.push(action.clone());
            }
            self.synced_actions.commit(self.storage.as_mut(), false)?;

            if response.uploaded != 0 {
                // Transfer completed local actions to synced state. Actions are only
                // appended so even if new local actions have been added since the
                // start of the sync, the uploaded ones will be in the same place.
                let mut local_iter = self.local_actions.iter();
                for _ in 0..response.uploaded {
                    if let Some(action) = local_iter.next() {
                        self.synced_solves
                            .resolve_action(action, &mut self.next_update_id);
                        self.synced_actions.push(action.clone());
                    } else {
                        break;
                    }
                }
                self.synced_actions.commit(self.storage.as_mut(), false)?;

                // Remove completed local actions
                let pos = local_iter.position();
                self.local_actions
                    .remove_starting_at(pos, self.storage.as_mut())?;
            }

            // Resolve local actions on top of the synced state. If there are actions that
            // are no longer required because they were already synced or no longer apply,
            // remove them.
            self.solves = self.synced_solves.clone();
            let mut new_actions = Vec::new();
            let mut has_rejected_actions = false;
            for action in self.local_actions.iter() {
                if self.solves.resolve_action(action, &mut self.next_update_id) {
                    new_actions.push(action.clone());
                } else {
                    has_rejected_actions = true;
                }
            }

            if has_rejected_actions {
                // If there were modifications to the local action list, reserialize the list
                // and replace the existing list with the new one.
                let mut new_list = ActionList::empty("local");
                for action in new_actions {
                    new_list.push(action);
                }
                new_list.commit(self.storage.as_mut(), true)?;
                self.local_actions.delete_bundles(self.storage.as_mut())?;
                self.local_actions = new_list;
            }

            self.update_id = self.next_update_id;
            self.next_update_id += 1;
        }

        // Update sync ID and commit to local database if changed
        if response.new_sync_id != self.sync_id {
            self.sync_id = response.new_sync_id;
            self.storage.put("sync_id", &self.sync_id.to_le_bytes())?;

            // If there are still local solves after receiving a new sync ID, resync to
            // apply the local solves to the new sync point
            if self.local_actions.has_actions() {
                self.current_sync = Some(SyncOperation::new(self.sync_request()));
            }
        }

        Ok(())
    }
}

impl SolveDatabase {
    fn new() -> Self {
        Self {
            solves: HashMap::new(),
            sessions: HashMap::new(),
            actions: HashSet::new(),
        }
    }

    fn add_solve_to_session(&mut self, solve: &String, session: &String, next_update_id: &mut u64) {
        let update_id = *next_update_id;
        *next_update_id += 1;
        let session = self
            .sessions
            .entry(session.clone())
            .or_insert_with(|| Session {
                id: session.clone(),
                name: None,
                solves: HashSet::new(),
                update_id,
            });
        session.solves.insert(solve.clone());
        session.update_id = update_id;
    }

    fn resolve_action(&mut self, action: &StoredAction, next_update_id: &mut u64) -> bool {
        // Ensure each action can only be resolved once (network drops during sync can
        // cause duplicate actions to stay in the local list)
        if !self.actions.insert(action.id.clone()) {
            return false;
        }

        match &action.action {
            Action::NewSolve(solve) => {
                self.solves.insert(solve.id.clone(), solve.clone());
                self.add_solve_to_session(&solve.id, &solve.session, next_update_id);
                true
            }
            Action::Penalty(solve, penalty) => match self.solves.get_mut(solve) {
                Some(solve) => {
                    solve.penalty = penalty.clone();
                    if let Some(session) = self.sessions.get_mut(&solve.session) {
                        session.update_id = *next_update_id;
                        *next_update_id += 1;
                    }
                    true
                }
                None => false,
            },
            Action::ChangeSession(solve_id, session_id) => match self.solves.get_mut(solve_id) {
                Some(solve) => {
                    match self.sessions.get_mut(&solve.session) {
                        Some(session) => {
                            session.solves.remove(solve_id);
                            session.update_id = *next_update_id;
                            *next_update_id += 1;
                        }
                        None => (),
                    };
                    solve.session = session_id.clone();
                    self.add_solve_to_session(solve_id, session_id, next_update_id);
                    true
                }
                None => false,
            },
            Action::MergeSessions(first, second) => {
                let second_solves = match self.sessions.get(second) {
                    Some(second) => second.solves.clone(),
                    None => return false,
                };
                match self.sessions.get_mut(first) {
                    Some(first) => {
                        for solve in second_solves {
                            if let Some(solve) = self.solves.get_mut(&solve) {
                                solve.session = first.id.clone();
                                first.solves.insert(solve.id.clone());
                            }
                        }
                        first.update_id = *next_update_id;
                        *next_update_id += 1;
                        self.sessions.remove(second);
                        true
                    }
                    None => false,
                }
            }
            Action::RenameSession(session, name) => match self.sessions.get_mut(session) {
                Some(session) => {
                    session.name = name.clone();
                    session.update_id = *next_update_id;
                    *next_update_id += 1;
                    true
                }
                None => false,
            },
            Action::DeleteSolve(solve_id) => match self.solves.get(solve_id) {
                Some(solve) => {
                    match self.sessions.get_mut(&solve.session) {
                        Some(session) => {
                            session.solves.remove(solve_id);
                            session.update_id = *next_update_id;
                            *next_update_id += 1;
                        }
                        None => (),
                    };
                    self.solves.remove(solve_id);
                    true
                }
                None => false,
            },
        }
    }
}

impl Session {
    pub fn sorted_solves(&self, history: &History) -> Vec<Solve> {
        let mut solves: Vec<Solve> = self
            .solves
            .iter()
            .filter_map(|solve_id| history.solves().get(solve_id).map(|solve| solve.clone()))
            .collect();

        // Sort solves by time, then by ID. There cannot be any equal solves so it
        // is fine to use the faster unstable sort here.
        solves.sort_unstable_by(|a, b| a.cmp(&b));
        solves
    }
}
