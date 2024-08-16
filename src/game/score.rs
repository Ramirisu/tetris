pub fn calculate_score(lines: usize, level: usize) -> usize {
    (level + 1)
        * match lines {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("can only clear lines between 1-4"),
        }
}
