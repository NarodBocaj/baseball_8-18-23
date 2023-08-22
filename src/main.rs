use rand::Rng;

fn main() {
    // let slug_avgs_by_idx = calc_best_idx_for_slugger(10_000_000);
    // println!("{:?}", slug_avgs_by_idx);

    let slug2_avgs_by_idx = calc_best_idxs_for_2sluggers(1_000_000);
    println!("{:?}", slug2_avgs_by_idx);
}

struct Game {
    first: u32,
    second: u32,
    third: u32,
    runs: u32,
    outs: u32,
    inning: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            first: 0,
            second: 0,
            third: 0,
            runs: 0,
            outs: 0,
            inning: 0,
        }
    }

    fn hit(&mut self, homerun: bool) {
        if homerun {
            self.runs += (self.first + self.second + self.third) + 1;
            self.first = 0;
            self.second = 0;
            self.third = 0;
        } 
        else {
            if self.third != 0 {
                self.runs += 1;
            }
            self.third = self.second;
            self.second = self.first;
            self.first = 1;
        }
    }

    fn out(&mut self) {
        self.outs += 1;
        if self.outs == 3{
            self.inning += 1;
            self.first = 0;
            self.second = 0;
            self.third = 0;
            self.outs = 0;
        }
    }
}


fn calc_best_idx_for_slugger(inning_count: u32) -> Vec<f32> {
    let mut rng = rand::thread_rng(); // Initialize the random number generator

    let mut slugger_avgs = Vec::new();

    for slugger_pos in 0..9 {
        let mut temp_run_in_innings = 0;
        let mut curr_batter = 0;
        let mut innings_played = 0;
        let mut game = Game::new();

        while innings_played < inning_count {
            let rand: f32 = rng.gen(); // Generate a random number between 0 and 1

            if slugger_pos == curr_batter {
                if rand < 1.0 / 10.0 {
                    game.hit(true);
                } else {
                    game.out();
                }
            } else {
                if rand < 1.0 / 3.0 {
                    game.hit(false);
                } else {
                    game.out();
                }
            }

            if game.inning == 1 {
                temp_run_in_innings += game.runs;
                curr_batter = 0;
                innings_played += 1;
                game = Game::new();
            } 
            else {
                curr_batter = if curr_batter < 8 { curr_batter + 1 } else { 0 };
            }
        }

        slugger_avgs.push(temp_run_in_innings as f32 / inning_count as f32);
    }

    return slugger_avgs
}

fn calc_best_idxs_for_2sluggers(game_count: u32) -> Vec<usize> {
    let mut rng = rand::thread_rng(); // Initialize the random number generator
    let mut max_run_avg = 0.0;
    let mut idx1 = 0;
    let mut idx2 = 0;

    let mut slugger_avgs = Vec::new();

    for slugger1_pos in 0..8 {
        for slugger2_pos in slugger1_pos + 1..9{
            let mut temp_run_in_games = 0;
            let mut curr_batter = 0;
            let mut games_played = 0;
            let mut game = Game::new();

            while games_played < game_count {
                let rand: f32 = rng.gen(); // Generate a random number between 0 and 1

                if slugger1_pos == curr_batter || slugger2_pos == curr_batter {
                    if rand < 1.0 / 10.0 {
                        game.hit(true);
                    } 
                    else {
                        game.out();
                    }
                } 
                else {
                    if rand < 1.0 / 3.0 {
                        game.hit(false);
                    } 
                    else {
                        game.out();
                    }
                }

                if game.inning == 9 {
                    temp_run_in_games += game.runs;
                    curr_batter = 0;
                    games_played += 1;
                    game = Game::new();
                } 
                else {
                    curr_batter = if curr_batter < 8 { curr_batter + 1 } else { 0 };
                }
            }
            let run_avg = temp_run_in_games as f32 / game_count as f32;
            slugger_avgs.push(vec![run_avg, slugger1_pos as f32, slugger2_pos as f32]);

            if run_avg > max_run_avg{
                max_run_avg = run_avg;
                idx1 = slugger1_pos;
                idx2 = slugger2_pos;
            }

        }
    }
    println!("{:?}", slugger_avgs);
    return vec![idx1, idx2]
}