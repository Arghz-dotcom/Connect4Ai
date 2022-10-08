use crate::alpha_beta6::position6;
use super::position6::Position6;
use std::cmp;
use std::fs;
use std::path::Path;

struct AlphaBetaLevel6Solver {
    node_count: u32,
    elapsed_time_ms: u32,
    column_order: [usize; 7],
    max_alpha_beta: [i8; position6::NBCOINS],
    pos: Position6,
}

impl AlphaBetaLevel6Solver {
    

    fn new(player: u8) -> Self {
        Self {
            node_count: 0,
            elapsed_time_ms: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            max_alpha_beta: Self::max_alpha_beta_init(),
            pos: Position6::new()
        }
    }

    fn max_alpha_beta_init() -> [i8; position6::NBCOINS] {
        let mut result = [0; position6::NBCOINS];
        for i in 0..position6::NBCOINS {
            result[i] = ((position6::NBCOINS + 1 - i)/2) as i8
        }
        return result;
    }

    fn negamax(&mut self, mut alpha: i8, mut beta: i8, depth: u8) -> i8 {
        self.node_count += 1;

        if depth == 0 || self.pos.moves == position6::NBCOINS {
            return 0;
        }

        for col in 0..position6::WIDTH {
            if self.pos.can_play(col) && self.pos.is_winning_move(col) {
                return self.max_alpha_beta[self.pos.moves];
            }
        }

        let max = self.max_alpha_beta[self.pos.moves];
        if beta > max {
            beta = max;
            if alpha >= beta {return beta;}
        }

        for real_col in self.column_order {
            if self.pos.can_play(real_col) {
                self.pos.play(real_col);
                let score = -self.negamax(-beta, -alpha, depth - 1);
                self.pos.unplay(real_col);
                if score >= beta {return score;}
                alpha = cmp::max(alpha, score);
            }
        }

        alpha
    }

    pub fn play_sequence(&mut self, seq: &str) {
        self.pos = Position6::new();
        self.pos.play_sequence(seq);
    }

    pub fn solve(&mut self, max_depth: u8) -> i8 {
        self.node_count = 0;
        let init_val = (position6::NBCOINS as i8) / 2;
        
        self.negamax(-init_val, init_val, max_depth)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_play() {
        let mut nb_read_rows = 10;
        let files_test = vec!["Test_L3_R1"];
        let mut solver = AlphaBetaLevel6Solver::new(1);
        for filename in files_test {
            let path = Path::new(r#"src\data"#).join(filename);
            let _display = path.to_str();
            let data = fs::read_to_string(path)
                                       .expect("Should have been able to read the file");
            let lines: Vec<&str> = data.split("\n").collect();
            nb_read_rows = cmp::min(nb_read_rows, lines.len());
            let total_node_count = 0;
            let total_time_ms = 0;
            for indexLine in 0..nb_read_rows {
                let line = lines[indexLine];
                let linedata: Vec<&str> = line.split(' ').collect();
                solver.play_sequence(linedata.first().unwrap());
                let score = solver.solve(25);
                /*totalTimeMs += solver.elapsedTimeMs
                totalNodeCount += solver.nodeCount*/

                assert_eq!(linedata[1], score.to_string());
            }

            /*let meanTime = (totalTimeMs/nb_read_rows).toFixed(2)
            let meanNbPos = (totalNodeCount/nb_read_rows).toFixed(2)
            let kpos = (totalNodeCount/totalTimeMs).toFixed(2)
            console.log("Mean time: %d, mean nb pos: %d, k pos/s: %d", meanTime, meanNbPos, kpos)*/
        }
    }
}