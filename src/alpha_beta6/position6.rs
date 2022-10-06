struct Position6 {
    pub heights: [u8; 7],
    pub moves: u8,
    bitboard: [u64; 2]
}

impl Position6 {
    const WIDTH: usize = 7;
    const HEIGHT: u8 = 6;
    const NBCOINS: u8 = 42;

    fn new() -> Position6 {
        Position6 {
            heights: [0, 7, 14, 21, 28, 35, 42],
            moves: 0,
            bitboard: [0, 0]
        }
    }
    
    pub fn can_play(&self, col: usize) -> bool {
        self.heights[col] % 7 != 6
    }

    fn is_winning_move(&self, col: usize) -> bool {
        let mut pos = self.bitboard[(self.moves & 1) as usize];
        pos ^= 1 << self.heights[col];

        //horizontal
        let mut m = pos & (pos >> 7);
        if m & (m >> 14) != 0 {return true;} 

        //diag1
        m = pos & (pos >> 6);
        if m & (m >> 12) != 0 {return true;}
        
        //diag2
        m = pos & (pos >> 8);
        if m & (m >> 16) != 0 {return true;}
        
        //vertical
        m = pos & (pos >> 1);
        if m & (m >> 2) != 0 {return true;}

        false
    }

    fn play(&mut self, col: usize) {
        let mov = 1 << self.heights[col];
        self.heights[col] = self.heights[col] + 1;
        self.bitboard[(self.moves & 1) as usize] ^= mov;
        self.moves = self.moves + 1;
    }

    fn unplay(&mut self, col: usize) {
        self.moves = self.moves - 1;
        self.heights[col] = self.heights[col] - 1;
        let mov = 1 << self.heights[col];
        self.bitboard[(self.moves & 1) as usize] ^= mov;
    }

    fn play_sequence (&mut self, seq: &str) -> usize {
        for (index_seq, seq_item) in seq.chars().enumerate() { 
            let col = (seq_item.to_digit(10).unwrap() - 1) as usize;
            if col >= Position6::WIDTH || !self.can_play(col) || self.is_winning_move(col) {
                return index_seq;
            }
            self.play(col)
        }
        return seq.chars().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_play() {
        let mut position = Position6::new();
        for r in 0..5 {
            for c in 0..7 {
                position.heights[c] = r;
                assert!(position.can_play(c))
            }
        }
    }

    #[test]
    fn play() {
        let mut position = Position6::new();
        position.play(0);
        assert_eq!(position.bitboard[0], 0b1);
        position.play(0);
        assert_eq!(position.bitboard[1], 0b10);
        position.play(0);
        assert_eq!(position.bitboard[0], 0b101);
        position.play(0);
        assert_eq!(position.bitboard[1], 0b1010);

        position = Position6::new();
        position.play(1);
        assert_eq!(position.bitboard[0], 0b10000000);
        position.play(1);
        assert_eq!(position.bitboard[1], 0b100000000);
    }

    #[test]
    fn unplay() {
        let mut position = Position6::new();
        position.play(0);
        assert_eq!(position.bitboard[0], 0b1);
        position.unplay(0);
        assert_eq!(position.bitboard[0], 0b0);
        position.play(1);
        assert_eq!(position.bitboard[0], 0b10000000);
        position.unplay(1);
        assert_eq!(position.bitboard[0], 0b0);

        for _i in 0..3 {
            position.play(0);
        }
            
        assert_eq!(position.bitboard[0], 0b101);
        assert_eq!(position.bitboard[1], 0b10);
        position.unplay(0);
        assert_eq!(position.bitboard[0], 0b1);
        assert_eq!(position.bitboard[1], 0b10);
        position.unplay(0);
        assert_eq!(position.bitboard[0], 0b1);
        assert_eq!(position.bitboard[1], 0b0);
    }

    #[test]
    fn is_winning_move_horizontal() {
        let mut position = Position6::new();
        let l = position.play_sequence("334323"); //col: 223213
        /*
        .......
        .......
        .......
        ..2....
        ..22...
        .111...
        */
        assert_eq!(l, 6);
    
        let mut result = position.is_winning_move(0);
        assert!(result);
        result = position.is_winning_move(4);
        assert!(result);
        result = position.is_winning_move(5);
        assert!(!result);
        result = position.is_winning_move(6);
        assert!(!result);
    }
    
    #[test]
    fn is_winning_move_diag1() {
        let mut position = Position6::new();
        let l = position.play_sequence("5443354556"); //col: 4332243445
        /*
        .......
        .......
        ....1..
        ...12..
        ..112..
        ..2212.
        */
        assert_eq!(l, 10);
    
        let result = position.is_winning_move(1);
        assert!(result);
    }
}