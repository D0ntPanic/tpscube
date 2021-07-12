use crate::action::{Action, ActionList, StoredAction};
use crate::common::{MoveSequence, Penalty, Solve, SolveType, TimedMoveSequence};
use crate::import::ImportedSession;
use crate::request::{SyncRequest, SyncResponse};
use crate::storage::{DeferredStorage, Storage};
use crate::sync::{SyncOperation, SyncStatus};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[cfg(feature = "native-storage")]
use dirs::data_local_dir;
#[cfg(feature = "native-storage")]
use std::path::Path;

const UNSYNCED: u32 = 0;

pub struct History {
    storage: DeferredStorage,
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
    settings: Settings,
}

#[derive(Clone, Copy)]
pub enum HistoryLoadProgress {
    InitializeDatabase,
    ReadSyncedActions,
    ReadLocalActions,
    ResolveDeltas(usize, usize),
}

#[derive(Clone)]
struct SolveDatabase {
    solve_map: SolveMap,
    sessions: HashMap<String, Session>,
    actions: HashSet<String>,
}

#[derive(Clone)]
struct SolveMap {
    solves: BTreeMap<SolveTimeAndId, Solve>,
    solve_times: HashMap<String, DateTime<Local>>,
}

#[derive(Clone)]
pub struct Session {
    id: String,
    name: Option<String>,
    solve_type: SolveType,
    solves: BTreeSet<SolveTimeAndId>,
    update_id: u64,
}

#[derive(Clone)]
struct SolveTimeAndId {
    time: DateTime<Local>,
    id: String,
}

#[derive(Clone)]
pub struct SolveIterator<'a> {
    solve: std::collections::btree_map::Iter<'a, SolveTimeAndId, Solve>,
}

#[derive(Clone)]
pub struct SessionSolveIterator<'a> {
    history: &'a History,
    solve: std::collections::btree_set::Iter<'a, SolveTimeAndId>,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    settings: HashMap<String, Vec<u8>>,
}

impl Default for HistoryLoadProgress {
    fn default() -> Self {
        Self::InitializeDatabase
    }
}

impl HistoryLoadProgress {
    pub fn approximate_percent_done(&self) -> f32 {
        match self {
            Self::InitializeDatabase => 0.0,
            Self::ReadSyncedActions => 5.0,
            Self::ReadLocalActions => 20.0,
            Self::ResolveDeltas(done, total) => {
                if *total == 0 {
                    100.0
                } else {
                    let frac = *done as f32 / *total as f32;
                    25.0 + frac * 75.0
                }
            }
        }
    }
}

impl History {
    #[cfg(feature = "native-storage")]
    pub async fn open() -> Result<Self> {
        let progress = Arc::new(Mutex::new(HistoryLoadProgress::default()));
        Self::open_with_progress(progress).await
    }

    #[cfg(feature = "native-storage")]
    pub async fn open_with_progress(progress: Arc<Mutex<HistoryLoadProgress>>) -> Result<Self> {
        let mut path =
            data_local_dir().ok_or_else(|| anyhow!("Local data directory not defined"))?;
        path.push("tpscube");
        path.push("solves");
        Self::open_at_with_progress(path, progress).await
    }

    #[cfg(feature = "native-storage")]
    pub async fn open_at<P: AsRef<Path>>(path: P) -> Result<Self> {
        let progress = Arc::new(Mutex::new(HistoryLoadProgress::default()));
        Self::open_at_with_progress(path, progress).await
    }

    #[cfg(feature = "native-storage")]
    pub async fn open_at_with_progress<P: AsRef<Path>>(
        path: P,
        progress: Arc<Mutex<HistoryLoadProgress>>,
    ) -> Result<Self> {
        // Open up the local database and read actions from it
        Self::open_with_storage(Storage::open(path.as_ref())?, progress).await
    }

    #[cfg(feature = "web-storage")]
    pub async fn open() -> Result<Self> {
        let progress = Arc::new(Mutex::new(HistoryLoadProgress::default()));
        Self::open_with_progress(progress).await
    }

    #[cfg(feature = "web-storage")]
    pub async fn open_with_progress(progress: Arc<Mutex<HistoryLoadProgress>>) -> Result<Self> {
        Self::open_with_storage(Storage::new().await?, progress).await
    }

    async fn open_with_storage(
        mut storage: Storage,
        progress: Arc<Mutex<HistoryLoadProgress>>,
    ) -> Result<Self> {
        *progress.lock().unwrap() = HistoryLoadProgress::ReadSyncedActions;
        let mut synced_actions = ActionList::load(&storage, "synced").await?;
        *progress.lock().unwrap() = HistoryLoadProgress::ReadLocalActions;
        let mut local_actions = ActionList::load(&storage, "local").await?;

        // Fetch sync information from local database
        let mut sync_key = match storage.get("sync_key").await? {
            Some(key) => SyncRequest::validate_sync_key(&String::from_utf8_lossy(&key)),
            None => None,
        };
        let mut sync_id = match storage.get("sync_id").await? {
            Some(raw_id) => Some(u32::from_le_bytes(
                raw_id.try_into().map_err(|_| anyhow!("Invalid sync ID"))?,
            )),
            None => None,
        };

        let current_session = match storage.get("session").await? {
            Some(session) => String::from_utf8_lossy(&session).into_owned(),
            None => {
                let session = Uuid::new_v4().to_simple().to_string();
                storage.put("session", session.as_bytes()).await?;
                session
            }
        };

        let settings = match storage.get("settings").await? {
            Some(settings) => serde_json::from_str(&String::from_utf8_lossy(&settings))?,
            None => Settings {
                settings: HashMap::new(),
            },
        };

        let storage = DeferredStorage::new(storage);

        if sync_key.is_none() || sync_id.is_none() {
            // No valid sync information in the database, create new sync information
            sync_key = Some(SyncRequest::new_sync_key());
            sync_id = Some(UNSYNCED);
            storage.put("sync_key", sync_key.as_ref().unwrap().as_bytes());
            storage.put("sync_id", &sync_id.unwrap().to_le_bytes());

            // If there was synced information that is now invalid, move it to local so
            // that can be synced under the new key and data loss is avoided.
            if synced_actions.has_actions() {
                local_actions.prepend(&mut synced_actions);
                local_actions.save_index(&storage);
                synced_actions.save_index(&storage);
            }
        }

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
            settings,
        };

        // Resolve actions to create solve and session lists
        let total = result.synced_actions.len() + result.local_actions.len();
        *progress.lock().unwrap() = HistoryLoadProgress::ResolveDeltas(0, total);
        let mut done = 0;

        for action in &result.synced_actions {
            result
                .synced_solves
                .resolve_action(action, &mut result.next_update_id);

            done += 1;
            if done % 4096 == 0 {
                *progress.lock().unwrap() = HistoryLoadProgress::ResolveDeltas(done, total);
            }
        }

        result.solves = result.synced_solves.clone();
        for action in &result.local_actions {
            result
                .solves
                .resolve_action(action, &mut result.next_update_id);

            done += 1;
            if done % 4096 == 0 {
                *progress.lock().unwrap() = HistoryLoadProgress::ResolveDeltas(done, total);
            }
        }

        Ok(result)
    }

    pub fn iter(&self) -> SolveIterator {
        SolveIterator {
            solve: self.solves.solve_map.solves.iter(),
        }
    }

    pub fn solve(&self, id: &str) -> Option<&Solve> {
        self.solves.solve(id)
    }

    pub fn sessions(&self) -> &HashMap<String, Session> {
        &self.solves.sessions
    }

    pub fn update_id(&self) -> u64 {
        self.update_id
    }

    pub fn sync_key(&self) -> &str {
        &self.sync_key
    }

    pub fn set_sync_key(&mut self, key: &str) -> Result<()> {
        // Set the key and make sure that any in progress syncs do not complete
        // on the new key.
        self.sync_key = key.into();
        self.sync_id = UNSYNCED;
        self.current_sync = None;
        self.last_sync_result = SyncStatus::NotSynced;

        // Move any synced actions to local so that they will be uploaded under
        // the new key.
        if self.synced_actions.has_actions() {
            self.local_actions.prepend(&mut self.synced_actions);
            self.local_actions.save_index(&self.storage);
            self.synced_actions.save_index(&self.storage);
            self.synced_solves = SolveDatabase::new();
        }

        self.storage.put("sync_key", self.sync_key.as_bytes());
        self.storage.put("sync_id", &self.sync_id.to_le_bytes());

        Ok(())
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

    pub fn new_session(&mut self) -> String {
        let session = Uuid::new_v4().to_simple().to_string();
        self.current_session = session.clone();
        self.storage.put("session", session.as_bytes());
        self.update_id = self.next_update_id;
        self.next_update_id += 1;
        session
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

    pub fn local_commit(&mut self) {
        self.local_actions.commit(&self.storage, false);
    }

    pub fn local_action_count(&self) -> usize {
        self.local_actions.len()
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

    pub fn sync_in_progress(&self) -> bool {
        self.current_sync.is_some()
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
                            self.resolve_sync(response);

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

    fn resolve_sync(&mut self, response: &SyncResponse) {
        if response.new_actions.len() != 0 || response.uploaded != 0 {
            // There are new actions, commit them to the synced state
            for action in &response.new_actions {
                self.synced_solves
                    .resolve_action(action, &mut self.next_update_id);
                self.synced_actions.push(action.clone());
            }
            self.synced_actions.commit(&self.storage, false);

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
                self.synced_actions.commit(&self.storage, false);

                // Remove completed local actions
                let pos = local_iter.position();
                self.local_actions.remove_before(pos, &self.storage);
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
                new_list.commit(&self.storage, true);
                self.local_actions.delete_bundles(&self.storage);
                self.local_actions = new_list;
            }

            self.update_id = self.next_update_id;
            self.next_update_id += 1;
        }

        // Update sync ID and commit to local database if changed
        if response.new_sync_id != self.sync_id {
            self.sync_id = response.new_sync_id;
            self.storage.put("sync_id", &self.sync_id.to_le_bytes());

            // If there are still local solves after receiving a new sync ID, resync to
            // apply the local solves to the new sync point
            if (response.new_actions.len() != 0 || response.uploaded != 0)
                && (self.local_actions.has_actions() || response.more_actions)
            {
                self.current_sync = Some(SyncOperation::new(self.sync_request()));
            }
        }
    }

    pub fn export(&self) -> Result<String> {
        // Sort sessions by solve time
        let mut sessions: Vec<&Session> = self.solves.sessions.values().collect();
        sessions.sort_unstable(); // Sessions are always unique

        let mut session_list = Vec::new();
        for session in sessions {
            let mut solve_list = Vec::new();
            for solve in session.iter(self) {
                let mut value = json!({
                    "id": solve.id,
                    "ok": if let Penalty::DNF = solve.penalty { false } else { true },
                    "penalty": match solve.penalty {
                        Penalty::None => 0,
                        Penalty::Time(time) => time,
                        Penalty::DNF => 0,
                    },
                    "scramble": solve.scramble.to_string(),
                    "time": solve.time,
                    "timestamp": solve.created.timestamp(),
                });
                if let Some(device) = &solve.device {
                    value
                        .as_object_mut()
                        .unwrap()
                        .insert("device".into(), json!(device));
                }
                if let Some(moves) = &solve.moves {
                    value
                        .as_object_mut()
                        .unwrap()
                        .insert("solve".into(), json!(moves.to_string()));
                }
                solve_list.push(value);
            }
            if solve_list.len() != 0 {
                session_list.push(json!({
                    "id": session.id,
                    "name": match &session.name {
                        Some(name) => &name,
                        None => ""
                    },
                    "solves": solve_list,
                    "type": session.solve_type.to_string(),
                }));
            }
        }

        Ok(serde_json::to_string_pretty(&json!({
            "sessions": session_list
        }))?)
    }

    pub fn import(&mut self, contents: String) -> Result<String> {
        // Import sessions and solves from the file contents
        let sessions = ImportedSession::import(contents)?;

        // Keep track of merge statistics
        let file_sessions = sessions.len();
        let mut file_solves = 0;
        let mut changed_session_count = 0;
        let mut new_session_count = 0;
        let mut changed_solve_count = 0;
        let mut new_solve_count = 0;

        for session in sessions {
            let mut existing = false;
            let mut changed = false;

            // Check for existing session
            if let Some(existing_session) = self.solves.sessions.get_mut(&session.id) {
                existing = true;

                // If name has changed, perform rename
                if existing_session.name != session.name {
                    if let Some(name) = &session.name {
                        self.rename_session(session.id.clone(), name.clone());
                        changed = true;
                    } else {
                        self.default_session_name(session.id.clone());
                        changed = true;
                    }
                }
            }

            // Merge solves in session
            file_solves += session.solves.len();
            for solve in &session.solves {
                // Check for existing solve
                if let Some(existing_solve) = self.solves.solve_map.solves.get(&SolveTimeAndId {
                    time: solve.created,
                    id: solve.id.clone(),
                }) {
                    // Check for modified penalty
                    if existing_solve.penalty != solve.penalty {
                        self.penalty(solve.id.clone(), solve.penalty.clone());
                        changed_solve_count += 1;
                        changed = true;
                    }
                } else {
                    // New solve
                    self.new_solve(solve.clone());
                    new_solve_count += 1;
                    changed = true;
                }
            }

            // If there is a new session and it has a name, give it the name now
            if !existing && changed {
                if let Some(name) = &session.name {
                    self.rename_session(session.id.clone(), name.clone());
                }
            }

            // Update session merge statistics
            if existing {
                if changed {
                    changed_session_count += 1;
                }
            } else if changed {
                new_session_count += 1;
            }
        }

        self.local_commit();

        // Import complete, return statistics about merge
        Ok(format!(
            "File contained {} solve(s) in {} session(s).\n\
            {} session(s) added.\n\
            {} session(s) modified.\n\
            {} solve(s) added.\n\
            {} solve(s) modified.",
            file_solves,
            file_sessions,
            new_session_count,
            changed_session_count,
            new_solve_count,
            changed_solve_count
        ))
    }

    pub fn auto_split_sessions(&mut self, max_gap_time: i64) -> usize {
        // Go through all sessions for organization
        let mut to_move = BTreeMap::new();
        let mut new_session_count = 0;
        for (_, session) in self.sessions() {
            if session.name.is_some() {
                continue;
            }

            let mut new_session = None;
            let mut prev_solve: Option<SolveTimeAndId> = None;

            // Go through solves in this session
            for solve in &session.solves {
                if let Some(prev) = &prev_solve {
                    let gap = solve.time - prev.time;
                    if gap.num_seconds() > max_gap_time {
                        // Found a gap larger than the timeout, create a new session
                        // for this solve and solves after it.
                        let session_id = Uuid::new_v4().to_simple().to_string();
                        new_session = Some(session_id);
                        new_session_count += 1;
                    }
                }

                // Add solve to list of solves to move to a new session if we
                // need to move it.
                if let Some(session_id) = &new_session {
                    to_move.insert(solve.id.clone(), session_id.clone());
                }

                prev_solve = Some(solve.clone());
            }
        }

        // Commit session changes
        for (solve_id, session_id) in to_move {
            self.change_session(solve_id, session_id);
        }

        self.local_commit();
        new_session_count
    }

    pub fn setting(&self, name: &str) -> Option<Vec<u8>> {
        if let Some(setting) = self.settings.settings.get(name) {
            Some(setting.clone())
        } else {
            None
        }
    }

    pub fn setting_as_bool(&self, name: &str) -> Option<bool> {
        if let Some(value) = self.setting(name) {
            if value.len() == 1 {
                return Some(value[0] != 0);
            }
        }
        None
    }

    pub fn setting_as_string(&self, name: &str) -> Option<String> {
        if let Some(value) = self.setting(name) {
            return Some(String::from_utf8_lossy(&value).into_owned());
        }
        None
    }

    pub fn setting_as_i64(&self, name: &str) -> Option<i64> {
        if let Some(value) = self.setting(name) {
            if value.len() == 8 {
                return Some(i64::from_le_bytes(value.try_into().unwrap()));
            }
        }
        None
    }

    pub fn set_setting(&mut self, name: &str, value: &[u8]) -> Result<()> {
        self.settings.settings.insert(name.into(), value.to_vec());
        self.storage.put(
            "settings",
            serde_json::to_string(&self.settings)?.as_bytes(),
        );
        Ok(())
    }

    pub fn set_bool_setting(&mut self, name: &str, value: bool) -> Result<()> {
        self.set_setting(name, &[if value { 1 } else { 0 }])
    }

    pub fn set_string_setting(&mut self, name: &str, value: &str) -> Result<()> {
        self.set_setting(name, value.as_bytes())
    }

    pub fn set_i64_setting(&mut self, name: &str, value: i64) -> Result<()> {
        self.set_setting(name, &value.to_le_bytes())
    }

    pub fn check_for_error(&self) -> Option<String> {
        self.storage.check_for_error()
    }
}

impl<'a> Iterator for SolveIterator<'a> {
    type Item = &'a Solve;

    fn next(&mut self) -> Option<Self::Item> {
        self.solve.next().map(|kv| kv.1)
    }
}

impl<'a> DoubleEndedIterator for SolveIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.solve.next_back().map(|kv| kv.1)
    }
}

impl PartialEq for SolveTimeAndId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SolveTimeAndId {}

impl PartialOrd for SolveTimeAndId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.time.cmp(&other.time) {
            Ordering::Equal => Some(self.id.cmp(&other.id)),
            ordering => Some(ordering),
        }
    }
}

impl Ord for SolveTimeAndId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl SolveMap {
    fn solve(&self, id: &str) -> Option<&Solve> {
        if let Some(time) = self.solve_times.get(id) {
            let key = SolveTimeAndId {
                time: time.clone(),
                id: id.into(),
            };
            self.solves.get(&key)
        } else {
            None
        }
    }

    fn solve_mut(&mut self, id: &str) -> Option<&mut Solve> {
        if let Some(time) = self.solve_times.get(id) {
            let key = SolveTimeAndId {
                time: time.clone(),
                id: id.into(),
            };
            self.solves.get_mut(&key)
        } else {
            None
        }
    }
}

impl SolveDatabase {
    fn new() -> Self {
        Self {
            solve_map: SolveMap {
                solves: BTreeMap::new(),
                solve_times: HashMap::new(),
            },
            sessions: HashMap::new(),
            actions: HashSet::new(),
        }
    }

    fn solve(&self, id: &str) -> Option<&Solve> {
        self.solve_map.solve(id)
    }

    fn add_solve_to_session(
        &mut self,
        solve: SolveTimeAndId,
        solve_type: SolveType,
        session: &String,
        next_update_id: &mut u64,
    ) {
        let update_id = *next_update_id;
        *next_update_id += 1;
        let session = self
            .sessions
            .entry(session.clone())
            .or_insert_with(|| Session {
                id: session.clone(),
                name: None,
                solve_type,
                solves: BTreeSet::new(),
                update_id,
            });
        session.solves.insert(solve);
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
                let key = SolveTimeAndId {
                    time: solve.created.clone(),
                    id: solve.id.clone(),
                };
                self.solve_map
                    .solve_times
                    .insert(solve.id.clone(), solve.created);
                self.solve_map.solves.insert(key.clone(), solve.clone());
                self.add_solve_to_session(key, solve.solve_type, &solve.session, next_update_id);
                true
            }
            Action::Penalty(solve, penalty) => match self.solve_map.solve_mut(solve) {
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
            Action::ChangeSession(solve_id, session_id) => match self.solve_map.solve_mut(solve_id)
            {
                Some(solve) => {
                    let key = SolveTimeAndId {
                        time: solve.created.clone(),
                        id: solve.id.clone(),
                    };
                    match self.sessions.get_mut(&solve.session) {
                        Some(session) => {
                            session.solves.remove(&key);
                            session.update_id = *next_update_id;
                            *next_update_id += 1;
                        }
                        None => (),
                    };
                    solve.session = session_id.clone();
                    let solve_type = solve.solve_type;
                    self.add_solve_to_session(key, solve_type, session_id, next_update_id);
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
                            if let Some(solve) = self.solve_map.solves.get_mut(&solve) {
                                solve.session = first.id.clone();
                                first.solves.insert(SolveTimeAndId {
                                    time: solve.created.clone(),
                                    id: solve.id.clone(),
                                });
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
            Action::DeleteSolve(solve_id) => match self.solve_map.solve(solve_id) {
                Some(solve) => {
                    let key = SolveTimeAndId {
                        time: solve.created.clone(),
                        id: solve_id.clone(),
                    };
                    match self.sessions.get_mut(&solve.session) {
                        Some(session) => {
                            session.solves.remove(&key);
                            session.update_id = *next_update_id;
                            *next_update_id += 1;
                        }
                        None => (),
                    };
                    self.solve_map.solve_times.remove(&key.id);
                    self.solve_map.solves.remove(&key);
                    true
                }
                None => false,
            },
        }
    }
}

impl Session {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn update_id(&self) -> u64 {
        self.update_id
    }

    pub fn len(&self) -> usize {
        self.solves.len()
    }

    pub fn iter<'a>(&'a self, history: &'a History) -> SessionSolveIterator<'a> {
        SessionSolveIterator {
            history,
            solve: self.solves.iter(),
        }
    }

    pub fn to_vec(&self, history: &History) -> Vec<Solve> {
        self.iter(history).cloned().collect()
    }

    pub fn last_solve_time(&self) -> Option<DateTime<Local>> {
        self.solves.iter().rev().next().map(|key| key.time)
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Session {}

impl PartialOrd for Session {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.last_solve_time().cmp(&other.last_solve_time()) {
            Ordering::Equal => Some(self.id.cmp(&other.id)),
            ordering => Some(ordering),
        }
    }
}

impl Ord for Session {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Iterator for SessionSolveIterator<'a> {
    type Item = &'a Solve;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(solve) = self.solve.next() {
                if let Some(solve) = self.history.solve(&solve.id) {
                    return Some(solve);
                }
            } else {
                return None;
            }
        }
    }
}

impl<'a> DoubleEndedIterator for SessionSolveIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(solve) = self.solve.next_back() {
                if let Some(solve) = self.history.solve(&solve.id) {
                    return Some(solve);
                }
            } else {
                return None;
            }
        }
    }
}
