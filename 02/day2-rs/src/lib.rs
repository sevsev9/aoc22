pub fn map_to_ints(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|chr| [(chr[0] as i32) - 65, (chr[2] as i32) - 88].to_vec())
        .collect::<Vec<Vec<i32>>>()
}

pub fn calc_scores_part1(plays: &Vec<Vec<i32>>) -> i32 {
    plays.iter().map(|play| play[1] + 1 + (((((3 + play[1] - play[0]) % 3) * 3) + 3) % 9) ).sum()
}

pub fn calc_scores_part2(plays: &Vec<Vec<i32>>) -> i32 {
    let l2d = [[3, 4, 8],[1, 5, 9],[2, 6, 7]];
    plays.iter().map(|play| l2d[play[0] as usize][play[1] as usize]).sum()
}