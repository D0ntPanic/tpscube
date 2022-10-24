use crate::common::{parse_move_string, parse_timed_move_string, Penalty, Solve, SolveType};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, TimeZone, Utc};
use serde_json::{Map, Value};
use std::str::FromStr;
use uuid::Uuid;

pub(crate) struct ImportedSession {
    pub id: String,
    pub name: Option<String>,
    pub solves: Vec<Solve>,
}

impl ImportedSession {
    pub fn import(contents: String) -> Result<Vec<ImportedSession>> {
        let contents = contents.trim();
        if contents.starts_with("{") {
            Self::import_json(contents)
        } else {
            Self::import_csv(contents)
        }
    }

    fn import_tpscube_json(value: &Map<String, Value>) -> Result<Vec<ImportedSession>> {
        // Parse list of sessions
        let mut sessions = Vec::new();
        let session_array = value
            .get("sessions")
            .ok_or_else(|| anyhow!("Sessions missing"))?
            .as_array()
            .ok_or_else(|| anyhow!("Session list not an array"))?;

        for session in session_array {
            let session = session
                .as_object()
                .ok_or_else(|| anyhow!("Session is not an object"))?;

            let session_id = session
                .get("id")
                .ok_or_else(|| anyhow!("Session is missing identifier"))?
                .as_str()
                .ok_or_else(|| anyhow!("Session identifier is not a string"))?;

            // Name is optional
            let name = if let Some(name) = session.get("name") {
                if let Some(name) = name.as_str() {
                    if name.len() == 0 {
                        None
                    } else {
                        Some(name)
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let solve_type_name = session
                .get("type")
                .ok_or_else(|| anyhow!("Session '{}' missing solve type", session_id))?
                .as_str()
                .ok_or_else(|| anyhow!("Session '{}' solve type is invalid", session_id))?;
            let solve_type = SolveType::from_str(solve_type_name).ok_or_else(|| {
                anyhow!(
                    "Unrecognized solve type '{}' in session '{}'",
                    solve_type_name,
                    session_id
                )
            })?;

            // Parse list of solves
            let mut solves = Vec::new();
            let solve_array = session
                .get("solves")
                .ok_or_else(|| anyhow!("Session '{}' is missing solve list", session_id))?
                .as_array()
                .ok_or_else(|| anyhow!("Session '{}' solve list is not an array", session_id))?;

            for solve in solve_array {
                let solve = solve
                    .as_object()
                    .ok_or_else(|| anyhow!("Solve in session '{}' is not an object", session_id))?;

                let id = solve
                    .get("id")
                    .ok_or_else(|| {
                        anyhow!(
                            "Solve in session '{}' does not have an identifier",
                            session_id
                        )
                    })?
                    .as_str()
                    .ok_or_else(|| anyhow!("Solve in session '{}' is not a string", session_id))?;

                let ok = solve
                    .get("ok")
                    .ok_or_else(|| anyhow!("Solve '{}' has no 'ok' flag", id))?
                    .as_bool()
                    .ok_or_else(|| anyhow!("Solve '{}' has invalid 'ok' flag", id))?;
                let penalty = solve
                    .get("penalty")
                    .ok_or_else(|| anyhow!("Solve '{}' is missing penalty", id))?
                    .as_u64()
                    .ok_or_else(|| anyhow!("Solve '{}' penalty is not an integer", id))?;

                let time = solve
                    .get("time")
                    .ok_or_else(|| anyhow!("Solve '{}' has no time", id))?
                    .as_u64()
                    .ok_or_else(|| anyhow!("Solve '{}' time is not an integer", id))?;

                let timestamp = solve
                    .get("timestamp")
                    .ok_or_else(|| anyhow!("Solve '{}' has no timestamp", id))?
                    .as_i64()
                    .ok_or_else(|| anyhow!("Solve '{}' has invalid timestamp", id))?;
                let timestamp = Local.timestamp(timestamp, 0);

                let scramble_string = solve
                    .get("scramble")
                    .ok_or_else(|| anyhow!("Solve '{}' has no scramble", id))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Solve '{}' has invalid scramble", id))?;
                let scramble = parse_move_string(&scramble_string)?;

                // Device is optional
                let device = if let Some(device) = solve.get("device") {
                    if let Some(device) = device.as_str() {
                        if device.len() == 0 {
                            None
                        } else {
                            Some(device)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Move list is optional
                let moves = if let Some(moves) = solve.get("solve") {
                    if let Some(moves) = moves.as_str() {
                        if moves.len() == 0 {
                            None
                        } else {
                            Some(parse_timed_move_string(moves)?)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Add solve to list
                solves.push(Solve {
                    id: id.into(),
                    solve_type,
                    session: session_id.into(),
                    scramble,
                    created: timestamp,
                    time: time as u32,
                    penalty: if ok {
                        if penalty > 0 {
                            Penalty::Time(penalty as u32)
                        } else {
                            Penalty::None
                        }
                    } else {
                        match penalty {
                            1 => Penalty::RecognitionDNF,
                            2 => Penalty::ExecutionDNF,
                            _ => Penalty::DNF,
                        }
                    },
                    device: device.map(|string| string.into()),
                    moves,
                });
            }

            sessions.push(ImportedSession {
                id: session_id.into(),
                name: name.map(|string| string.into()),
                solves,
            });
        }

        Ok(sessions)
    }

    fn import_cstimer_json(value: &Map<String, Value>) -> Result<Vec<ImportedSession>> {
        let mut sessions = Vec::new();

        // Decode session data. It's JSON converted to a string and placed inside of
        // JSON. Seems unnecessary but it is what it is.
        let session_data = value
            .get("properties")
            .ok_or_else(|| anyhow!("Properties missing"))?
            .as_object()
            .ok_or_else(|| anyhow!("Properties are not an object"))?
            .get("sessionData")
            .ok_or_else(|| anyhow!("Session data missing"))?
            .as_str()
            .ok_or_else(|| anyhow!("Session data is not a string"))?;
        let session_data: Value = serde_json::from_str(session_data)?;
        let session_data = session_data
            .as_object()
            .ok_or_else(|| anyhow!("Deserialized session data is not an object"))?;

        // Loop through object keys looking for session data
        for (key, value) in value.iter() {
            if !key.starts_with("session") {
                continue;
            }

            let session_id = Uuid::new_v4().to_simple().to_string();

            // Look up session options. The key for the session options is the number after "session".
            // More inconsistency but it is what it is.
            let session_index_str = key.split_at(7).1;
            let session_data = session_data
                .get(session_index_str)
                .ok_or_else(|| {
                    anyhow!("Session data for session '{}' not found", session_index_str)
                })?
                .as_object()
                .ok_or_else(|| {
                    anyhow!(
                        "Session data for session '{}' not an object",
                        session_index_str
                    )
                })?;
            let options = session_data
                .get("opt")
                .ok_or_else(|| anyhow!("Session '{}' does not have options", session_index_str))?
                .as_object()
                .ok_or_else(|| {
                    anyhow!(
                        "Session '{}' has options that are not an object",
                        session_index_str
                    )
                })?;

            // Check solve type in options. The absense of a solve type means standard 3x3x3.
            let solve_type = if let Some(solve_type_str) = options.get("scrType") {
                let solve_type_str = solve_type_str.as_str().ok_or_else(|| {
                    anyhow!("Session '{}' has invalid solve type", session_index_str)
                })?;
                match solve_type_str {
                    "333" => SolveType::Standard3x3x3,
                    _ => continue,
                }
            } else {
                SolveType::Standard3x3x3
            };

            // Parse solve list
            let solve_list = value.as_array().ok_or_else(|| {
                anyhow!(
                    "Solve list for session '{}' is not an array",
                    session_index_str
                )
            })?;

            let mut solves = Vec::new();
            for solve in solve_list {
                let solve = solve.as_array().ok_or_else(|| {
                    anyhow!("Solve in session '{}' is not an array", session_index_str)
                })?;

                // Parse time and penalty
                let time_array = solve
                    .get(0)
                    .ok_or_else(|| anyhow!("Solve time array missing"))?
                    .as_array()
                    .ok_or_else(|| anyhow!("Solve time is not inside an array"))?;
                let penalty = time_array
                    .get(0)
                    .ok_or_else(|| anyhow!("Solve penalty missing"))?
                    .as_i64()
                    .ok_or_else(|| anyhow!("Solve penalty is not an integer"))?;
                let penalty = match penalty {
                    -1 => Penalty::DNF,
                    0 => Penalty::None,
                    time => Penalty::Time(time as u32),
                };
                let time = time_array
                    .get(1)
                    .ok_or_else(|| anyhow!("Solve time missing"))?
                    .as_u64()
                    .ok_or_else(|| anyhow!("Solve time is not an integer"))?
                    as u32;

                // Parse scramble
                let scramble = solve
                    .get(1)
                    .ok_or_else(|| anyhow!("Scramble missing"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Scramble is not a string"))?;
                let scramble = parse_move_string(&scramble)?;

                // Parse timestamp
                let timestamp = solve
                    .get(3)
                    .ok_or_else(|| anyhow!("Timestamp missing"))?
                    .as_i64()
                    .ok_or_else(|| anyhow!("Timestamp is not an integer"))?;
                let id = format!("cstimer:{}", timestamp);
                let timestamp = Local.timestamp(timestamp, 0);

                // Parse move sequence (optional)
                let moves = if let Some(moves) = solve.get(4) {
                    let moves = moves
                        .as_array()
                        .ok_or_else(|| anyhow!("Move data is not an array"))?;
                    if let Some(moves) = moves.get(0) {
                        let moves = moves
                            .as_str()
                            .ok_or_else(|| anyhow!("Move list is not a string"))?;
                        Some(parse_timed_move_string(moves)?)
                    } else {
                        None
                    }
                } else {
                    None
                };

                solves.push(Solve {
                    id,
                    solve_type,
                    session: session_id.clone(),
                    scramble,
                    created: timestamp,
                    time,
                    penalty,
                    device: None,
                    moves,
                });
            }

            if solves.len() > 0 {
                sessions.push(ImportedSession {
                    id: session_id.clone(),
                    name: None,
                    solves,
                });
            }
        }

        Ok(sessions)
    }

    fn import_json(contents: &str) -> Result<Vec<ImportedSession>> {
        let value: Value = serde_json::from_str(contents)?;
        let value = value
            .as_object()
            .ok_or_else(|| anyhow!("Invalid JSON object"))?;
        if value.contains_key("properties") {
            Self::import_cstimer_json(value)
        } else if value.contains_key("sessions") {
            Self::import_tpscube_json(value)
        } else {
            Err(anyhow!("Solve data format not recognized"))
        }
    }

    fn import_csv(contents: &str) -> Result<Vec<ImportedSession>> {
        let mut reader = csv::Reader::from_reader(contents.as_bytes());

        // Parse header and find the needed fields
        let mut id_index = None;
        let mut date_index = None;
        let mut dnf_index = None;
        let mut penalty_index = None;
        let mut time_index = None;
        let mut device_index = None;
        let mut scramble_index = None;
        let mut solution_index = None;
        for (idx, field) in reader.headers()?.iter().enumerate() {
            match field {
                "id" => id_index = Some(idx),
                "date" => date_index = Some(idx),
                "dnf" => dnf_index = Some(idx),
                "one_turn_away_two_second_penalty" => penalty_index = Some(idx),
                "timer_time" => time_index = Some(idx),
                "device_name" => device_index = Some(idx),
                "scramble" => scramble_index = Some(idx),
                "solution" => solution_index = Some(idx),
                _ => (),
            }
        }

        // Validate fields are present
        let id_index = id_index.ok_or_else(|| anyhow!("id field missing"))?;
        let date_index = date_index.ok_or_else(|| anyhow!("date field missing"))?;
        let dnf_index = dnf_index.ok_or_else(|| anyhow!("dnf field missing"))?;
        let penalty_index = penalty_index.ok_or_else(|| anyhow!("penalty field missing"))?;
        let time_index = time_index.ok_or_else(|| anyhow!("time field missing"))?;
        let device_index = device_index.ok_or_else(|| anyhow!("device field missing"))?;
        let scramble_index = scramble_index.ok_or_else(|| anyhow!("scramble field missing"))?;
        let solution_index = solution_index.ok_or_else(|| anyhow!("solution field missing"))?;

        // Parse all solves
        let mut solves = Vec::new();
        let session_id = Uuid::new_v4().to_simple().to_string();
        for solve in reader.records() {
            let solve = solve?;

            // Pull out fields
            let id = solve
                .get(id_index)
                .ok_or_else(|| anyhow!("id missing in solve"))?;
            let date = solve
                .get(date_index)
                .ok_or_else(|| anyhow!("date missing in solve"))?;
            let dnf = solve
                .get(dnf_index)
                .ok_or_else(|| anyhow!("dnf missing in solve"))?;
            let penalty = solve
                .get(penalty_index)
                .ok_or_else(|| anyhow!("penalty missing in solve"))?;
            let time = solve
                .get(time_index)
                .ok_or_else(|| anyhow!("time missing in solve"))?;
            let device = solve
                .get(device_index)
                .ok_or_else(|| anyhow!("device missing in solve"))?;
            let scramble = solve
                .get(scramble_index)
                .ok_or_else(|| anyhow!("scramble missing in solve"))?;
            let solution = solve
                .get(solution_index)
                .ok_or_else(|| anyhow!("solution missing in solve"))?;

            // Parse fields
            let date: DateTime<Local> = Utc.datetime_from_str(date, "%Y-%m-%d %H:%M:%S %Z")?.into();
            let dnf = match dnf {
                "true" => true,
                "false" => false,
                _ => return Err(anyhow!("Invalid dnf field")),
            };
            let penalty = match penalty {
                "true" => true,
                "false" => false,
                _ => return Err(anyhow!("Invalid penalty field")),
            };
            let penalty = if dnf {
                Penalty::DNF
            } else if penalty {
                Penalty::Time(2000)
            } else {
                Penalty::None
            };
            let time = if dnf { 0 } else { u32::from_str(time)? };
            let scramble = parse_move_string(&scramble)?;
            let solution = if dnf {
                None
            } else {
                Some(parse_timed_move_string(
                    &solution.replace("[", "@").replace("]", ""),
                )?)
            };

            solves.push(Solve {
                id: id.into(),
                solve_type: SolveType::Standard3x3x3,
                session: session_id.clone(),
                scramble,
                created: date,
                time,
                penalty,
                device: Some(device.into()),
                moves: solution,
            });
        }

        let mut sessions = Vec::new();
        if solves.len() > 0 {
            sessions.push(ImportedSession {
                id: session_id.clone(),
                name: None,
                solves,
            });
        }

        Ok(sessions)
    }
}
