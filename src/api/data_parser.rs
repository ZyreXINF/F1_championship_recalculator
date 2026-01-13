use std::collections::{BTreeMap, HashMap};
use toml::Value;
use crate::model::driver::Driver;
use crate::model::race_format::RaceKind;

pub async fn fetch_and_merge_races(championship_year: &u16) -> Result<BTreeMap<u8, Vec<Value>>, Box<dyn std::error::Error>> {
    let mut offset = 0;
    let limit = 100;
    let mut merged_races: BTreeMap<u8, Vec<Value>> = BTreeMap::new();
    loop {
        let url = format!("https://api.jolpi.ca/ergast/f1/{}/results.json?limit={}&offset={}", championship_year, limit, offset);
        let body = reqwest::get(&url).await?.text().await?;
        let value: Value = serde_json::from_str(&body)?;

        let total: u32 = value["MRData"]["total"].as_str().unwrap().parse()?;
        let races = value["MRData"]["RaceTable"]["Races"]
            .as_array()
            .unwrap();

        if races.is_empty() {
            break;
        }

        for race in races {
            let round: u8 = race["round"].as_str().unwrap().parse()?;
            let results = race["Results"].as_array().unwrap();
            merged_races
                .entry(round)
                .or_default()
                .extend(results.iter().cloned());
        }

        offset += limit;
        if offset >= total {
            break;
        }
    }

    Ok(merged_races)
}

pub async fn fetch_sprint_races(championship_year: &u16, ) -> Result<BTreeMap<u8, Vec<Value>>, Box<dyn std::error::Error>> {
    let mut offset = 0;
    let limit = 100;
    let mut merged: BTreeMap<u8, Vec<Value>> = BTreeMap::new();
    loop {
        let url = format!("https://api.jolpi.ca/ergast/f1/{}/sprint.json?limit={}&offset={}", championship_year, limit, offset);
        let body = reqwest::get(&url).await?.text().await?;
        let value: Value = serde_json::from_str(&body)?;

        let total: u32 = value["MRData"]["total"].as_str().unwrap().parse()?;
        let races = value["MRData"]["RaceTable"]["Races"]
            .as_array()
            .unwrap();

        if races.is_empty() {
            break;
        }

        for race in races {
            let round: u8 = race["round"].as_str().unwrap().parse()?;
            if let Some(results) = race.get("SprintResults").and_then(|v| v.as_array()) {
                merged
                    .entry(round)
                    .or_default()
                    .extend(results.iter().cloned());
            }
        }

        offset += limit;
        if offset >= total {
            break;
        }
    }

    Ok(merged)
}

pub async fn request_drivers_results(championship_year: &u16, ) -> Result<Vec<Driver>, Box<dyn std::error::Error>> {
    let mut drivers: HashMap<String, Driver> = HashMap::new();

    let races = fetch_and_merge_races(championship_year).await?;
    for results in races.values() {
        for result in results {
            process_result(&mut drivers, result, RaceKind::GrandPrix)?;
        }
    }

    let sprint_races = fetch_sprint_races(championship_year).await?;
    for results in sprint_races.values() {
        for result in results {
            process_result(&mut drivers, result, RaceKind::Sprint)?;
        }
    }

    Ok(drivers.into_values().collect())
}

fn process_result(drivers: &mut HashMap<String, Driver>, result: &Value, race_type: RaceKind, ) -> Result<(), Box<dyn std::error::Error>> {
    let pos: u8 = result["position"].as_str().unwrap().parse()?;

    let driver = &result["Driver"];
    let name = format!(
        "{} {}",
        driver["givenName"].as_str().unwrap(),
        driver["familyName"].as_str().unwrap()
    );

    let fastest_lap = result.get("FastestLap")
        .and_then(|fl| fl.get("rank"))
        .and_then(|r| r.as_str()) == Some("1");

    let finished = result["status"].as_str() == Some("Finished") || result["status"].as_str() == Some("Lapped");

    let is_on_pole = result["grid"].as_str() == Some("1");

    let won = if pos == 1 {true} else {false};

    let entry = drivers.entry(name.clone()).or_insert(Driver {
        finish_history: Vec::new(),
        sprint_finish_history: Vec::new(),
        full_name: name,
        wins: 0,
        sprint_wins: 0,
        fastest_laps: 0,
        sprint_fastest_laps: 0,
        poles: 0,
        sprint_poles: 0,
    });

    match race_type {
        RaceKind::GrandPrix => {
            if finished {
                entry.finish_history.push(pos);
            }
            if fastest_lap {
                entry.fastest_laps += 1;
            }
            if is_on_pole{
                entry.poles += 1;
            }
            if won {
                entry.wins += 1;
            }
        }
        RaceKind::Sprint => {
            if finished {
                entry.sprint_finish_history.push(pos);
            }
            if fastest_lap {
                entry.sprint_fastest_laps += 1;
            }
            if is_on_pole{
                entry.sprint_poles += 1;
            }
            if won {
                entry.sprint_wins += 1;
            }
        }
    }

    Ok(())
}