use anyhow::{anyhow as e, Result};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
struct StatsWithParsedNames {
    #[serde(flatten)]
    stats: mvdparser::KtxstatsV3,
    parsed_names: HashMap<String, String>,
    unicode_names: HashMap<String, String>,
    parsed_teams: HashMap<String, String>,
}

fn main() -> Result<()> {
    let demo_path = std::env::args().nth(1).expect("ERROR: demo path required");

    // read demo data (bytes)
    let Ok(demo_data) = fs::read(&demo_path) else {
        return Err(e!("ERROR: unable to read {:?}", &demo_path));
    };

    // parse ktxstats
    let Ok(stats) = mvdparser::ktxstats_v3(&demo_data) else {
        return Err(e!("ERROR: unable extract ktxstats"));
    };

    // Create HashMaps to store parsed and Unicode names and teams
    let mut parsed_names: HashMap<String, String> = HashMap::new();
    let mut unicode_names: HashMap<String, String> = HashMap::new();
    let mut parsed_teams: HashMap<String, String> = HashMap::new();

    // Add parsed_name and unicode_name for each player
    for player in &stats.players {
        let parsed_name = quake_text::unicode::to_ascii(&player.name);
        parsed_names.insert(player.name.clone(), parsed_name);
        let unicode_name = player.name.chars().map(|c| format!("\\u{:04x}", c as u32)).collect::<String>();
        unicode_names.insert(player.name.clone(), unicode_name);
    }

    // Add parsed_team for each team
    for team in &stats.teams {
        let parsed_team = quake_text::unicode::to_ascii(team);
        parsed_teams.insert(team.clone(), parsed_team);
    }

    // Create the combined structure
    let stats_with_parsed_names = StatsWithParsedNames {
        stats,
        parsed_names,
        unicode_names,
        parsed_teams,
    };

    // Serialize the combined structure
    let stats_str = serde_json::to_string_pretty(&stats_with_parsed_names)?;

    // a) print embedded ktxstats
    println!("{}", stats_str);

    // b) write embedded ktxstats to JSON file
    {
        let demo_json_path = Path::new(&demo_path).with_extension("ktxstats.json");

        if fs::write(demo_json_path, stats_str).is_err() {
            return Err(e!("ERROR: unable to write ktxstats to JSON file"));
        };
    }

    Ok(())
}
