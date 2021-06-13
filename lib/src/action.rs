use crate::action_generated;
use crate::common::{Move, Penalty, Solve, SolveType, TimedMove};
use anyhow::{anyhow, Result};
use chrono::{Local, TimeZone};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

#[cfg(feature = "storage")]
use crate::index_generated;
#[cfg(feature = "storage")]
use crate::storage::Storage;

#[cfg(feature = "storage")]
const TARGET_BUNDLE_SIZE: usize = 65536;

#[derive(Clone, Debug)]
pub enum Action {
    NewSolve(Solve),
    Penalty(String, Penalty),
    ChangeSession(String, String),
    MergeSessions(String, String),
    RenameSession(String, Option<String>),
    DeleteSolve(String),
}

#[derive(Clone, Debug)]
pub struct StoredAction {
    pub id: String,
    pub action: Action,
}

#[cfg(feature = "storage")]
pub(crate) struct ActionList {
    name: &'static str,
    archive: Vec<ActionBundle>,
    current: ActionBundle,
}

#[cfg(feature = "storage")]
pub(crate) struct ActionListIterator<'a> {
    list: &'a ActionList,
    archive_iter: Option<std::iter::Enumerate<std::slice::Iter<'a, ActionBundle>>>,
    action_iter: Option<std::iter::Enumerate<std::slice::Iter<'a, StoredAction>>>,
    position: ActionListPosition,
}

#[cfg(feature = "storage")]
#[derive(Clone)]
pub(crate) struct ActionListPosition {
    archive_idx: Option<usize>,
    action_idx: usize,
}

#[cfg(feature = "storage")]
struct ActionBundle {
    id: String,
    actions: Vec<StoredAction>,
    present_in_index: bool,
}

impl StoredAction {
    pub fn new(action: Action) -> Self {
        Self {
            id: Uuid::new_v4().to_simple().to_string(),
            action,
        }
    }

    pub fn serialize<'a: 'b, 'b>(
        &self,
        builder: &'b mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<crate::action_generated::Action<'a>> {
        // Create the appropriate flatbuffer table for the action
        let (contents, contents_type) = match &self.action {
            Action::NewSolve(solve) => {
                let id = builder.create_string(&solve.id);
                let session = builder.create_string(&solve.session);
                let scramble: Vec<u8> = solve.scramble.iter().map(|mv| *mv as u8).collect();
                let scramble = builder.create_vector(&scramble);
                let (penalty_type, penalty) = match solve.penalty {
                    Penalty::None => (action_generated::Penalty::NONE, None),
                    Penalty::Time(time) => (
                        action_generated::Penalty::TimePenalty,
                        Some(
                            action_generated::TimePenalty::create(
                                builder,
                                &action_generated::TimePenaltyArgs { time },
                            )
                            .as_union_value(),
                        ),
                    ),
                    Penalty::DNF => (
                        action_generated::Penalty::DNFPenalty,
                        Some(
                            action_generated::DNFPenalty::create(
                                builder,
                                &action_generated::DNFPenaltyArgs {},
                            )
                            .as_union_value(),
                        ),
                    ),
                };
                let device = solve
                    .device
                    .as_ref()
                    .map(|device| builder.create_string(&device));
                let moves = solve.moves.as_ref().map(|moves| {
                    let mut move_list = Vec::new();
                    for mv in moves {
                        move_list.push(action_generated::TimedMove::new(
                            mv.move_() as u8,
                            mv.time(),
                        ));
                    }
                    builder.create_vector(&move_list)
                });
                let mut solve_builder = action_generated::NewSolveActionBuilder::new(builder);
                solve_builder.add_id(id);
                solve_builder.add_solve_type(solve.solve_type as u8);
                solve_builder.add_session(session);
                solve_builder.add_scramble(scramble);
                solve_builder.add_created(solve.created.timestamp());
                solve_builder.add_time(solve.time);
                solve_builder.add_penalty_type(penalty_type);
                if let Some(penalty) = penalty {
                    solve_builder.add_penalty(penalty);
                }
                if let Some(device) = device {
                    solve_builder.add_device(device);
                }
                if let Some(moves) = moves {
                    solve_builder.add_moves(moves);
                }

                (
                    solve_builder.finish().as_union_value(),
                    action_generated::ActionContents::NewSolveAction,
                )
            }
            Action::Penalty(solve, penalty) => {
                let solve = builder.create_string(&solve);
                let (penalty_type, penalty) = match penalty {
                    Penalty::None => (action_generated::Penalty::NONE, None),
                    Penalty::Time(time) => (
                        action_generated::Penalty::TimePenalty,
                        Some(
                            action_generated::TimePenalty::create(
                                builder,
                                &action_generated::TimePenaltyArgs { time: *time },
                            )
                            .as_union_value(),
                        ),
                    ),
                    Penalty::DNF => (
                        action_generated::Penalty::DNFPenalty,
                        Some(
                            action_generated::DNFPenalty::create(
                                builder,
                                &action_generated::DNFPenaltyArgs {},
                            )
                            .as_union_value(),
                        ),
                    ),
                };
                let mut penalty_builder = action_generated::PenaltyActionBuilder::new(builder);
                penalty_builder.add_solve(solve);
                penalty_builder.add_penalty_type(penalty_type);
                if let Some(penalty) = penalty {
                    penalty_builder.add_penalty(penalty);
                }

                (
                    penalty_builder.finish().as_union_value(),
                    action_generated::ActionContents::PenaltyAction,
                )
            }
            Action::ChangeSession(solve, session) => {
                let solve = Some(builder.create_string(&solve));
                let session = Some(builder.create_string(&session));
                let action = action_generated::ChangeSessionAction::create(
                    builder,
                    &action_generated::ChangeSessionActionArgs { solve, session },
                )
                .as_union_value();

                (
                    action,
                    action_generated::ActionContents::ChangeSessionAction,
                )
            }
            Action::MergeSessions(first, second) => {
                let first = Some(builder.create_string(&first));
                let second = Some(builder.create_string(&second));
                let action = action_generated::MergeSessionsAction::create(
                    builder,
                    &action_generated::MergeSessionsActionArgs { first, second },
                )
                .as_union_value();

                (
                    action,
                    action_generated::ActionContents::MergeSessionsAction,
                )
            }
            Action::RenameSession(session, name) => {
                let session = builder.create_string(&session);
                let name = name.as_ref().map(|name| builder.create_string(&name));
                let mut rename_builder = action_generated::RenameSessionActionBuilder::new(builder);
                rename_builder.add_session(session);
                if let Some(name) = name {
                    rename_builder.add_name(name);
                }

                (
                    rename_builder.finish().as_union_value(),
                    action_generated::ActionContents::RenameSessionAction,
                )
            }
            Action::DeleteSolve(solve) => {
                let solve = Some(builder.create_string(&solve));
                let action = action_generated::DeleteSolveAction::create(
                    builder,
                    &action_generated::DeleteSolveActionArgs { solve },
                )
                .as_union_value();

                (action, action_generated::ActionContents::DeleteSolveAction)
            }
        };

        let id = builder.create_string(&self.id);

        // Create the final action using the action contents union
        let mut action_builder = action_generated::ActionBuilder::new(builder);
        action_builder.add_id(id);
        action_builder.add_contents_type(contents_type);
        action_builder.add_contents(contents);
        action_builder.finish()
    }

    pub fn serialize_list(actions: &[Self]) -> Result<Vec<u8>> {
        let mut builder = FlatBufferBuilder::new();

        // Serialize each action in the bundle
        let mut serialized = Vec::new();
        for action in actions {
            serialized.push(action.serialize(&mut builder));
        }

        // Create a serialized action list and finalize it
        let actions = Some(builder.create_vector(&serialized));
        let actions = action_generated::ActionList::create(
            &mut builder,
            &action_generated::ActionListArgs { actions },
        );
        builder.finish(actions, None);

        // Save serialized action bundle to the database
        Ok(builder.finished_data().to_vec())
    }

    pub fn deserialize(action: action_generated::Action) -> Option<Self> {
        let id = match action.id() {
            Some(id) => id.to_string(),
            None => return None,
        };
        match action.contents_type() {
            action_generated::ActionContents::NewSolveAction => {
                let action = match action.contents_as_new_solve_action() {
                    Some(action) => action,
                    None => return None,
                };
                let solve_id = match action.id() {
                    Some(id) => id.to_string(),
                    None => return None,
                };
                let solve_type: SolveType = match action.solve_type().try_into() {
                    Ok(solve_type) => solve_type,
                    _ => return None,
                };
                let session = match action.session() {
                    Some(session) => session.to_string(),
                    None => return None,
                };
                let scramble = match action.scramble() {
                    Some(scramble) => {
                        let mut moves: Vec<Move> = Vec::new();
                        for byte in scramble {
                            let mv = match Move::try_from(*byte) {
                                Ok(mv) => mv,
                                _ => return None,
                            };
                            moves.push(mv);
                        }
                        moves
                    }
                    None => return None,
                };
                let created = Local.timestamp(action.created(), 0);
                let time = action.time();
                let penalty = match action.penalty_type() {
                    action_generated::Penalty::TimePenalty => {
                        let penalty = match action.penalty_as_time_penalty() {
                            Some(penalty) => penalty,
                            None => return None,
                        };
                        let time = penalty.time();
                        Penalty::Time(time)
                    }
                    action_generated::Penalty::DNFPenalty => Penalty::DNF,
                    _ => Penalty::None,
                };
                let device = match action.device() {
                    Some(device) => Some(device.to_string()),
                    None => None,
                };
                let moves = match action.moves() {
                    Some(moves) => {
                        let mut result = Vec::new();
                        for timed_move in moves {
                            let mv = match Move::try_from(timed_move.move_()) {
                                Ok(mv) => mv,
                                _ => return None,
                            };
                            result.push(TimedMove::new(mv, timed_move.time()));
                        }
                        Some(result)
                    }
                    None => None,
                };
                Some(Self {
                    id,
                    action: Action::NewSolve(Solve {
                        id: solve_id,
                        solve_type,
                        session,
                        scramble,
                        created,
                        time,
                        penalty,
                        device,
                        moves,
                    }),
                })
            }
            action_generated::ActionContents::PenaltyAction => {
                let action = match action.contents_as_penalty_action() {
                    Some(action) => action,
                    None => return None,
                };
                let solve = match action.solve() {
                    Some(solve) => solve.to_string(),
                    None => return None,
                };
                let penalty = match action.penalty_type() {
                    action_generated::Penalty::TimePenalty => {
                        let penalty = match action.penalty_as_time_penalty() {
                            Some(penalty) => penalty,
                            None => return None,
                        };
                        let time = penalty.time();
                        Penalty::Time(time)
                    }
                    action_generated::Penalty::DNFPenalty => Penalty::DNF,
                    _ => Penalty::None,
                };
                Some(Self {
                    id,
                    action: Action::Penalty(solve, penalty),
                })
            }
            action_generated::ActionContents::ChangeSessionAction => {
                let action = match action.contents_as_change_session_action() {
                    Some(action) => action,
                    None => return None,
                };
                let solve = match action.solve() {
                    Some(solve) => solve.to_string(),
                    None => return None,
                };
                let session = match action.session() {
                    Some(session) => session.to_string(),
                    None => return None,
                };
                Some(Self {
                    id,
                    action: Action::ChangeSession(solve, session),
                })
            }
            action_generated::ActionContents::MergeSessionsAction => {
                let action = match action.contents_as_merge_sessions_action() {
                    Some(action) => action,
                    None => return None,
                };
                let first = match action.first() {
                    Some(first) => first.to_string(),
                    None => return None,
                };
                let second = match action.second() {
                    Some(second) => second.to_string(),
                    None => return None,
                };
                Some(Self {
                    id,
                    action: Action::MergeSessions(first, second),
                })
            }
            action_generated::ActionContents::RenameSessionAction => {
                let action = match action.contents_as_rename_session_action() {
                    Some(action) => action,
                    None => return None,
                };
                let session = match action.session() {
                    Some(session) => session.to_string(),
                    None => return None,
                };
                let name = action.name().map(|name| name.to_string());
                Some(Self {
                    id,
                    action: Action::RenameSession(session, name),
                })
            }
            action_generated::ActionContents::DeleteSolveAction => {
                let action = match action.contents_as_delete_solve_action() {
                    Some(action) => action,
                    None => return None,
                };
                let solve = match action.solve() {
                    Some(solve) => solve.to_string(),
                    None => return None,
                };
                Some(Self {
                    id,
                    action: Action::DeleteSolve(solve),
                })
            }
            _ => None,
        }
    }

    pub fn deserialize_list(data: &[u8]) -> Result<Vec<Self>> {
        let action_list = action_generated::root_as_action_list(data)?;
        if let Some(action_list) = action_list.actions() {
            let mut actions = Vec::new();
            for action in action_list.iter() {
                if let Some(action) = StoredAction::deserialize(action) {
                    actions.push(action);
                }
            }
            Ok(actions)
        } else {
            Err(anyhow!("Actions not present in list"))
        }
    }
}

#[cfg(feature = "storage")]
impl ActionList {
    pub fn empty(name: &'static str) -> Self {
        Self {
            name,
            archive: Vec::new(),
            current: ActionBundle::new(),
        }
    }

    pub fn load(storage: &dyn Storage, name: &'static str) -> Result<Self> {
        if let Some(data) = storage.get(name)? {
            let index = index_generated::root_as_action_list_index(&data)?;
            if let Some(lists) = index.lists() {
                // Load each bundle referenced in the index
                let mut bundles = Vec::new();
                for bundle in lists {
                    bundles.push(ActionBundle::load(storage, bundle)?);
                }

                if bundles.len() != 0 {
                    // Current bundle is the last in the list. The rest are the archived bundles.
                    // New actions will be added to the last bundle until it reaches the target
                    // maximum size.
                    let current = bundles.remove(bundles.len() - 1);
                    return Ok(Self {
                        name,
                        archive: bundles,
                        current,
                    });
                }
            }
        }

        Ok(Self::empty(name))
    }

    pub fn push(&mut self, action: StoredAction) {
        self.current.actions.push(action);
    }

    pub fn commit(&mut self, storage: &mut dyn Storage, always_write: bool) -> Result<()> {
        if self.current.actions.len() == 0 {
            if always_write {
                self.save_index(storage)?;
            }

            // Nothing to save
            return Ok(());
        }

        // Save the current bundle to storage
        let bundle_complete = self.current.save(storage)?;

        // Make sure the current bundle is present in the index, otherwise
        // it will not be visible on reload
        if !self.current.present_in_index || always_write {
            self.save_index(storage)?;
            self.current.present_in_index = true;
        }

        // When saving the size of the bundle is checked against the desired
        // bundle size. If the current bundle is at the desired bundle size,
        // start a new bundle on the next action.
        if bundle_complete {
            let mut bundle = ActionBundle::new();
            std::mem::swap(&mut bundle, &mut self.current);
            self.archive.push(bundle);
        }

        storage.flush();
        Ok(())
    }

    pub fn save_index(&self, storage: &mut dyn Storage) -> Result<()> {
        // Serialize list of action bundles to a flatbuffer
        let mut builder = FlatBufferBuilder::new();
        let mut lists: Vec<&str> = Vec::with_capacity(self.archive.len() + 1);
        for archive in &self.archive {
            lists.push(&archive.id);
        }
        if self.current.actions.len() != 0 {
            lists.push(&self.current.id);
        }
        let lists = builder.create_vector_of_strings(&lists);
        let index = crate::index_generated::ActionListIndex::create(
            &mut builder,
            &crate::index_generated::ActionListIndexArgs { lists: Some(lists) },
        );
        builder.finish(index, None);

        // Save serialized index to the database
        storage.put(self.name, builder.finished_data())?;
        Ok(())
    }

    pub fn iter(&self) -> ActionListIterator {
        ActionListIterator {
            list: self,
            archive_iter: Some(self.archive.iter().enumerate()),
            action_iter: None,
            position: ActionListPosition {
                archive_idx: Some(0),
                action_idx: 0,
            },
        }
    }

    pub fn has_actions(&self) -> bool {
        self.iter().next().is_some()
    }

    /// Prepends actions from `other` into the archive of `self`, leaving `other`
    /// with no actions.
    pub fn prepend(&mut self, other: &mut ActionList) {
        let mut new_index = Vec::new();
        std::mem::swap(&mut new_index, &mut other.archive);
        let mut old_current = ActionBundle::new();
        std::mem::swap(&mut old_current, &mut other.current);
        if old_current.present_in_index {
            new_index.push(old_current);
        }
        new_index.append(&mut self.archive);
        std::mem::swap(&mut new_index, &mut self.archive);
    }

    pub fn remove_starting_at(
        &mut self,
        position: ActionListPosition,
        storage: &mut dyn Storage,
    ) -> Result<()> {
        let mut index_changed = false;
        let local_remove_idx = if let Some(archive_idx) = position.archive_idx {
            // Deletion position is in archive, check to see if it is past the end of
            // the valid archives
            if archive_idx < self.archive.len() {
                // Position is within a valid archive
                let archive = &mut self.archive[archive_idx];
                let mut archive_updated = false;
                if position.action_idx < archive.actions.len() {
                    // There are actions to remove in this archive
                    let _ = archive.actions.split_off(position.action_idx);
                    archive_updated = true;
                }

                // Check to see if archive still has actions in it
                if archive.actions.len() == 0 {
                    // No more actions, delete this archive and any after it
                    for removed_archive in self.archive.split_off(archive_idx) {
                        removed_archive.delete(storage)?;
                    }
                    index_changed = true;
                } else {
                    // This archive still has some actions in it, save the revised archive
                    if archive_updated {
                        archive.save(storage)?;
                    }

                    // Remove any archives after this one
                    for removed_archive in self.archive.split_off(archive_idx + 1) {
                        removed_archive.delete(storage)?;
                        index_changed = true;
                    }
                }
            }

            // Remove all actions in the current bundle, as these are after the archives
            0
        } else {
            // Position is in current bundle, start removing at the position's action
            position.action_idx
        };

        // Check to see if action is past end of action list
        if local_remove_idx < self.current.actions.len() {
            // Action is within list, remove the actions after position
            let _ = self.current.actions.split_off(local_remove_idx);
            if self.current.actions.len() == 0 {
                // There are no more actions in this bundle, delete it and create a new
                // empty bundle.
                self.current.delete(storage)?;
                self.current = ActionBundle::new();
                index_changed = true;
            } else {
                // Save the updated action list
                self.current.save(storage)?;
            }
        }

        // If there were changes to the bundle list, save them now
        if index_changed {
            self.save_index(storage)?;
        }

        Ok(())
    }

    pub fn delete_bundles(&self, storage: &mut dyn Storage) -> Result<()> {
        for archive in &self.archive {
            archive.delete(storage)?;
        }
        if self.current.present_in_index {
            self.current.delete(storage)?;
        }
        Ok(())
    }
}

#[cfg(feature = "storage")]
impl<'a> IntoIterator for &'a ActionList {
    type Item = &'a StoredAction;
    type IntoIter = ActionListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(feature = "storage")]
impl<'a> ActionListIterator<'a> {
    pub fn position(&self) -> ActionListPosition {
        self.position.clone()
    }
}

#[cfg(feature = "storage")]
impl<'a> Iterator for ActionListIterator<'a> {
    type Item = &'a StoredAction;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Go to next action in current bundle
            if let Some(action_iter) = &mut self.action_iter {
                if let Some((idx, action)) = action_iter.next() {
                    self.position = ActionListPosition {
                        archive_idx: self.position.archive_idx,
                        action_idx: idx,
                    };
                    return Some(action);
                }
            }

            if let Some(archive_iter) = &mut self.archive_iter {
                if let Some((idx, archive)) = archive_iter.next() {
                    // Go to next archived bundle
                    self.action_iter = Some(archive.actions.iter().enumerate());
                    self.position = ActionListPosition {
                        archive_idx: Some(idx),
                        action_idx: 0,
                    };
                } else {
                    // After last archived bundle, go to current bundle
                    self.action_iter = Some(self.list.current.actions.iter().enumerate());
                    self.archive_iter = None;
                    self.position = ActionListPosition {
                        archive_idx: None,
                        action_idx: 0,
                    };
                }
            } else {
                // If we get here, `archive_iter` is None and we are done iterating
                // the current bundle. Iteration is complete.
                self.position = ActionListPosition {
                    archive_idx: None,
                    action_idx: self.list.current.actions.len(),
                };
                return None;
            }
        }
    }
}

#[cfg(feature = "storage")]
impl ActionBundle {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_simple().to_string(),
            actions: Vec::new(),
            present_in_index: false,
        }
    }

    fn load(storage: &dyn Storage, id: &str) -> Result<Self> {
        if let Some(data) = storage.get(id)? {
            let actions = StoredAction::deserialize_list(&data)?;
            Ok(Self {
                id: id.to_string(),
                actions,
                present_in_index: true,
            })
        } else {
            Ok(Self {
                id: id.to_string(),
                actions: Vec::new(),
                present_in_index: true,
            })
        }
    }

    fn save(&self, storage: &mut dyn Storage) -> Result<bool> {
        let data = StoredAction::serialize_list(&self.actions)?;
        storage.put(&self.id, &data)?;
        Ok(data.len() >= TARGET_BUNDLE_SIZE)
    }

    fn delete(&self, storage: &mut dyn Storage) -> Result<()> {
        storage.delete(&self.id)
    }
}
