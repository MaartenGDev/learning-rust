use std::collections::HashMap;
use std::slice::Chunks;
use permutohedron::LexicalPermutation;
use std::borrow::Borrow;

#[derive(Clone, Debug, Hash)]
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

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.team_a == other.team_a && self.team_b == other.team_b
    }
}

impl Eq for Match {}


pub fn generate() {
    let mut teams = ["teamA", "TeamB", "TeamC", "TeamD"];

    let mut permutations = Vec::new();

    loop {
        permutations.push(teams.to_vec());
        if !teams.next_permutation() {
            break;
        }
    }
    let amount_of_matches_against_same_team = 1;
    let minimum_week_count = 3;

    let mut permutation_index = 0;

    let mut matches_per_week: HashMap<u32, Vec<&Vec<&str>>> = HashMap::new();

    for week_number in 0..minimum_week_count + 1 {
        for permutation in &permutations {
            let week_permutations = matches_per_week.entry(week_number).or_insert([].to_vec());

            week_permutations.push(permutation);

            permutation_index += 1;
        }
    }


    let mut buckets: HashMap<u32, &mut Vec<&mut Vec<&Vec<&str>>>> = HashMap::new();

    let mut schedules: HashMap<u32, Vec<&[&str]>> = HashMap::new();
    let mut schedule_id = 0;
    let mut bucket_id = 0;

    // use first week as starting point
    for &round in &matches_per_week[&0] {
        let round_bucket = buckets.entry(bucket_id).or_insert(&mut vec![]);

        round_bucket.push(&mut vec![round]);

        for (next_week_nr, rounds_in_next_week) in &matches_per_week {
            for round_in_next_week in rounds_in_next_week {
                if round_in_next_week == &round {
                    continue;
                }


                for schedule in round_bucket.iter_mut() {
//                    schedule.push(round_in_next_week);

                    round_bucket.push(schedule);
                }
            }
        }
        bucket_id += 1;
    }

    println!("{:#?}", schedules);


    let mut teams_played_in_week: Vec<Match> = vec![];
    let mut matches_by_variant: HashMap<Match, Vec<Match>> = HashMap::new();


    let mut first_round_options: Vec<[Match; 2]> = vec![];


    // 0-2
    for (_, team_a) in teams.iter().enumerate() {
        for (_, team_b) in teams.iter().enumerate() {
            if team_a.eq(team_b) {
                continue;
            }

            let mut matches_based_on_source = matches_by_variant.entry( Match::new(team_a, team_b)).or_insert([].to_vec());


            for (_, nested_team_a) in teams.iter().enumerate() {
                for (_, nested_team_b) in teams.iter().enumerate() {
                    if nested_team_a.eq(nested_team_b) {
                        continue;
                    }

                    if team_a.eq(nested_team_a) || team_a.eq(nested_team_b) || team_b.eq(nested_team_a) || team_b.eq(nested_team_b) {
                        continue;
                    }

                    first_round_options.push([Match::new(team_a, team_b), Match::new(nested_team_a, nested_team_b)]);
                    matches_based_on_source.push(Match::new(nested_team_a, nested_team_b));
                }
            }
        }
    }
//
//    println!("{:#?}", first_round_options)
}

