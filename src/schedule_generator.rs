use std::collections::HashMap;
use std::slice::Chunks;

#[derive(Clone, Debug)]
struct Match {
    team_a: String,
    team_b: String,
}

impl Match {
    fn new(team_a: &str, team_b: &str) -> Match {
        Match {
            team_a: String::from(team_a),
            team_b: String::from(team_b),
        }
    }
}

pub fn generate() {
    let teams = ["teamA", "teamB", "TeamC"];


    let mut teams_played_in_week: Vec<&str> = vec![];
    let mut matches: Vec<Match> = vec![];
    let mut matches_by_variant: HashMap<u32, Vec<Match>> = HashMap::new();

    // 0-2
    for (_, team_a) in teams.iter().enumerate() {
        for (variant_id, team_b) in teams.iter().enumerate() { // 0-2
            if team_a.eq(team_b) {
                continue;
            }

            let variant_collection = matches_by_variant.entry(variant_id as u32).or_insert([].to_vec());
            variant_collection.push(Match::new(team_a, team_b));
        }
    }
    let mut match_options: HashMap<&u32, Vec<&Match>> = HashMap::new();

    for (team_a_id, match_options_for_team_a) in matches_by_variant.iter() {
        let variant_collection = match_options.entry(team_a_id).or_insert([].to_vec());

        for match_team_a in match_options_for_team_a {
            variant_collection.push(match_team_a);

            for (team_b_id, match_options_for_team_b) in matches_by_variant.iter() {
                if team_a_id.eq(team_b_id) {
                    continue;
                }

                for match_team_b in match_options_for_team_b.iter() {
                    variant_collection.push(match_team_b);
                }
            }
        }
    }

    let mut match_options_with_unique_start: HashMap<&u32, Chunks<&Match>> = HashMap::new();

    for (team_a_id, match_options_for_team_a) in match_options.iter() {
        match_options_with_unique_start.insert(team_a_id, match_options_for_team_a.chunks(3));
    }
    println!("{:#?}", matches_by_variant);

    for (team_id, matches_chunk) in match_options_with_unique_start {
        for (chunk_id, matches_in_chunk) in matches_chunk.enumerate() {
            for match_to_play in matches_in_chunk {
                println!("round: {}, variant: {} -> {} vs {}", team_id,chunk_id, match_to_play.team_a, match_to_play.team_b)
            }
            println!();
        }

        println!();
        println!();
    }
}

