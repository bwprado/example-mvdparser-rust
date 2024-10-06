use anyhow::{anyhow as e, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

    // ktxstats as string
    let stats_str = serde_json::to_string_pretty(&stats)?;

    // a) print embedded ktxstats
    println!("{}", stats_str);

    // b) write embedded ktxstats to JSON file
    {
        let demo_json_path = Path::new(&demo_path).with_extension("ktxstats.json");

        if fs::write(demo_json_path, stats_str).is_err() {
            return Err(e!("ERROR: unable to write ktxstats to JSON file"));
        };
    }

    // collect login per player name
    let mut login_per_name = HashMap::new();

    for player in stats.players {
        let name_as_ascii = quake_text::unicode::to_ascii(&player.name);
        login_per_name.insert(name_as_ascii, player.login.to_string());
    }

    // c) print login per user
    println!("{:?}", login_per_name);

    // d) write login per player name to JSON
    {
        let logins_json_path = Path::new(&demo_path).with_extension("logins.json");
        let logins_json_str = serde_json::to_string_pretty(&login_per_name)?;

        if fs::write(logins_json_path, logins_json_str).is_err() {
            return Err(e!("ERROR: unable to write login per user to JSON file"));
        };
    }

    Ok(())
}
