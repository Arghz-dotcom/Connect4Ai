mod Position6;

struct alphaBetaLevel6Solver {
    nodeCount: u32,
    elapsedTimeMs: u32,
    columnOrder: [u8; 7],
    maxAlphaBeta: [u8; Position6.NBCOINS],
    pos: Position6
}

impl alphaBetaLevel6Solver {
    fn new(player: u8) -> alphaBetaLevel6Solver {
        for i in 0..Position6.NBCOINS {
            maxAlphaBeta[i] = (Position6.NBCOINS + 1 - i) / 2;
        }
        alphaBetaLevel6Solver {
            nodeCount: 0,
            elapsedTimeMs: 0,
            columnOrder: [3, 2, 4, 1, 5, 0, 6],
        }
    }

    fn negamax(alpha: i8, beta: i8, depth: u8) -> i8 {
        nodeCount = nodeCount + 1;

        if (depth == 0 || pos.moves == Position6.NBCOINS) {
            return 0;
        }

        for col in 0..Position6.WIDTH {
            if (pos.canPlay(col) && pos.isWinningMove(col)) {
                return this.maxAlphaBeta[pos.moves];
            }
        }

        let max = maxAlphaBeta[pos.moves];
        if (beta > max) {
            beta = max;
            if (alpha >= beta) {return beta;}
        }

        for realCol in columnOrder {
            if(pos.canPlay(realCol)) {
                pos.play(realCol);
                let score = -negamax(-beta, -alpha, depth - 1);
                pos.unplay(realCol);
                if (score >= beta) {return score;}
                alpha = Math.max(alpha, score);
            }
        }

        return alpha;
    }
}