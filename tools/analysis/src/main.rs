use chrono::Local;
use clap::{App, Arg};
use std::str::FromStr;
use std::time::Duration;
use tpscube_core::{CFOPPartialAnalysis, CubeWithSolution, History};

#[tokio::main]
async fn main() {
    let matches = App::new("TPS Cube Analysis")
        .arg(Arg::with_name("hours").long("hours").takes_value(true))
        .arg(Arg::with_name("days").long("days").takes_value(true))
        .arg(Arg::with_name("weeks").long("weeks").takes_value(true))
        .arg(Arg::with_name("all").long("all"))
        .get_matches();

    let json = matches.value_of("json");
    let all = matches.is_present("all");

    let mut duration = Duration::from_secs(0);
    if let Some(hours) = matches.value_of("hours") {
        let hours = u64::from_str(hours).unwrap();
        duration += Duration::from_secs(3600 * hours);
    }
    if let Some(days) = matches.value_of("days") {
        let days = u64::from_str(days).unwrap();
        duration += Duration::from_secs(3600 * 24 * days);
    }
    if let Some(weeks) = matches.value_of("weeks") {
        let weeks = u64::from_str(weeks).unwrap();
        duration += Duration::from_secs(3600 * 24 * 7 * weeks);
    }

    let history = History::open().await.unwrap();

    let mut last_session = None;
    for solve in history.iter().rev() {
        if solve.moves.is_none() {
            continue;
        }

        if !all && duration.as_secs() > 0 {
            if (Local::now() - solve.created).to_std().unwrap() > duration {
                continue;
            }
        } else if !all {
            if let Some(last_session) = &last_session {
                if &solve.session != last_session {
                    break;
                }
            } else {
                last_session = Some(solve.session.clone());
            }
        }

        let solution: Option<CubeWithSolution> = solve.into();
        if let Some(solution) = solution {
            let analysis = CFOPPartialAnalysis::analyze(&solution);
            println!("Solve in {}ms\n{}", solve.time, analysis);
        }
    }
}
