use std::collections::{BTreeMap, HashMap};
use toml::Value;
use crate::model::driver::Driver;

pub async fn fetch_and_merge_races(championship_year: &u16) -> Result<BTreeMap<u8, Vec<Value>>, Box<dyn std::error::Error>> {
    let mut offset = 0;
    let limit = 100;

    let mut merged_races: BTreeMap<u8, Vec<Value>> = BTreeMap::new();
    loop {
        let url = format!("https://api.jolpi.ca/ergast/f1/{}/results.json?limit={}&offset={}",
                          championship_year, limit, offset);
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

pub async fn request_drivers_results(championship_year: &u16) -> Result<Vec<Driver>, Box<dyn std::error::Error>> {
    let races = fetch_and_merge_races(championship_year).await?;
    let mut drivers: HashMap<String, Driver> = HashMap::new();

    for (round, results) in races {
        for result in results {
            let pos: u8 = result["position"].as_str().unwrap().parse()?;

            let driver = &result["Driver"];
            let name = format!(
                "{} {}",
                driver["givenName"].as_str().unwrap(),
                driver["familyName"].as_str().unwrap()
            );

            let set_fastest_lap = result.get("FastestLap")
                .and_then(|fastest_lap| fastest_lap.get("rank"))
                .and_then(|rank| rank.as_str()) == Some("1");

            let entry = drivers.entry(name.clone()).or_insert(Driver {
                finish_history: Vec::new(),
                full_name: name.clone(),
                fastest_laps: 0,
            });
            let finished = result.get("status").and_then(|status| status.as_str()) == Some("Finished");
            if finished {
                entry.finish_history.push(pos);
            }
            if set_fastest_lap {
                entry.fastest_laps += 1;
            }
        }
    }
    let driver_results: Vec<Driver> = drivers.into_values().collect();
    Ok(driver_results)
}