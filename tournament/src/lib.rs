use std::collections::HashMap;

// This is the ugliest code of all time.  But it works.  I will come back
// and make this better.

const ALASKA: &str = "Allegoric Alaskans";
const DONKEYS: &str = "Devastating Donkeys";
const CALIFORNIA: &str = "Courageous Californians";
const BADGERS: &str = "Blithering Badgers";

const WIN: &str = "win";
const LOSS: &str = "loss";
const DRAW: &str = "draw";

struct Team {
    name: &'static str,
    wins: usize,
    losses: usize,
    draws: usize,
    num_matches: usize,
}

impl Team {
    fn new(name: &'static str) -> Self {
        Team {
            name,
            wins: 0,
            losses: 0,
            draws: 0,
            num_matches: 0,
        }
    }

    fn points(&self) -> usize {
        self.wins * 3 + self.draws
    }

    fn status_line(&self) -> String {
        format!(
            "\n{:31}|  {} |  {} |  {} |  {} |  {}",
            self.name,
            self.num_matches,
            self.wins,
            self.draws,
            self.losses,
            self.points(),
        )
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.points() == other.points() && self.name == other.name
    }
}

impl Eq for Team {}

impl PartialOrd for Team {
    // If the points are even we sort by name.  However, if not
    // we sort by points in descending order.  Gross!
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.points().eq(&other.points()) {
            Some(self.name.cmp(other.name))
        } else {
            Some(other.points().cmp(&self.points()))
        }
    }
}

impl Ord for Team {
    // If the points are even we sort by name.  However, if not
    // we sort by points in descending order.  Gross!
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.points().eq(&other.points()) {
            self.name.cmp(other.name)
        } else {
            other.points().cmp(&self.points())
        }
    }
}

pub fn tally(match_results: &str) -> String {
    // The simplest method I could think of right off the bat
    // is to use a Map.
    let mut teams_map = HashMap::from([
        (CALIFORNIA, Team::new(CALIFORNIA)),
        (BADGERS, Team::new(BADGERS)),
        (ALASKA, Team::new(ALASKA)),
        (DONKEYS, Team::new(DONKEYS)),
    ]);

    let mut match_occurred = false;
    let result_lines = match_results.split('\n');
    for result_line in result_lines {
        let mut result_cells = result_line.split(';');
        let team1 = result_cells.next();
        let team2 = result_cells.next();
        let result = result_cells.next();

        // First make sure we got all three elements or calling unwrap below will panic
        if team1.is_none() || team2.is_none() || result.is_none() {
            continue;
        }

        // Process the results based on the three elements we parsed
        let result = result.unwrap();
        process_team(team1.unwrap(), result, true, &mut teams_map);
        process_team(team2.unwrap(), result, false, &mut teams_map);
        match_occurred = true; // If we got this far at least one match occurred
    }

    // We always return this header to the caller even if there were no matches...
    let mut result_str = String::from("Team                           | MP |  W |  D |  L |  P");

    // We only output results if a match actually happened
    if match_occurred {
        let mut team_vec = teams_map.values().collect::<Vec<&Team>>();
        team_vec.sort();
        for team in team_vec {
            if team.num_matches > 0 {
                result_str += &team.status_line();
            }
        }
    }
    result_str
}

// This is gross... but the first bool tells us which team was first in the result line and
// which was second.
fn process_team(team_name: &str, result: &str, first: bool, team_map: &mut HashMap<&str, Team>) {
    let team = team_map.get_mut(team_name).unwrap();
    // I will simplify this later
    match first {
        true => match result {
            WIN => {
                team.wins += 1;
            }
            LOSS => {
                team.losses += 1;
            }
            DRAW => {
                team.draws += 1;
            }
            _ => {}
        },
        false => match result {
            WIN => {
                team.losses += 1;
            }
            LOSS => {
                team.wins += 1;
            }
            DRAW => {
                team.draws += 1;
            }
            _ => {}
        },
    }
    team.num_matches += 1;
}
