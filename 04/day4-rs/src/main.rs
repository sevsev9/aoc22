use std::cmp::{max, min};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let binding = contents.lines().collect::<Vec<&str>>();

    // print the amount of pairs that are fully contained in one or the other
    println!("Contained pairs: {}", fully_contained(&binding));

    // print the amount of pairs that are overlapping at all
    print!("Overlapping parirs: {}", overlapping(&binding))
}

fn parse_lines(lines: &Vec<&str>) -> Vec<Vec<Vec<i32>>> {
    lines.iter()
        // split into pairs of elves by ","
        .map(|line| line.split(",").collect::<Vec<&str>>())
        // parse into arrays of two integers with [range start, range end]
        .map(|pairs| pairs.iter()
            // "1-2" -> [1,2]
            .map(|range| range.split("-")
                .map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            ).collect::<Vec<Vec<i32>>>()
        ).collect()
}

fn fully_contained(lines: &Vec<&str>) -> usize {
    // filter out pairs where the range of one is fully contained in the other
    parse_lines(lines).iter().filter(|pair| is_in_range(pair)).collect::<Vec<&Vec<Vec<i32>>>>().len()
}

fn overlapping(lines: &Vec<&str>) -> usize {
    parse_lines(lines).iter().filter(|pair| overlaps(pair)).collect::<Vec<&Vec<Vec<i32>>>>().len()
}

fn is_in_range(pair: &Vec<Vec<i32>>) -> bool {
    return pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1] || pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1];
}

fn overlaps(pair: &Vec<Vec<i32>>) -> bool {
    max(pair[0][0], pair[1][0]) <= min(pair[0][1], pair[1][1])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_range() {
        let mut contained_range: Vec<Vec<i32>> = Vec::new();
        contained_range.push([2, 8].to_vec());
        contained_range.push([3, 7].to_vec());

        let mut not_contained_range: Vec<Vec<i32>> = Vec::new();
        not_contained_range.push([1, 4].to_vec());
        not_contained_range.push([5, 8].to_vec());

        assert_eq!(is_in_range(&contained_range), true);
        assert_eq!(is_in_range(&not_contained_range), false);
    }

    #[test]
    fn test_part_1() {
        let mut lines: Vec<&str> = Vec::new();
        lines.push("38-41,38-38");
        lines.push("18-65,18-65");
        lines.push("1-3,4-39");
        lines.push("41-42,40-40");
        lines.push("1-90,89-90");
        lines.push("30-84,31-85");
        lines.push("2-98,64-97");
        lines.push("75-75,15-76");
        lines.push("81-81,22-81");
        lines.push("29-92,30-30");
        lines.push("95-95,14-95");

        assert_eq!(fully_contained(&lines), 8);
    }

    #[test]
    fn test_part_2() {
        let mut lines: Vec<&str> = Vec::new();
        lines.push("38-41,38-38");  // overlaps
        lines.push("18-65,18-65");  // overlaps
        lines.push("1-3,4-39");     // does not overlap
        lines.push("41-42,40-40");  // does not overlap
        lines.push("1-90,89-90");   // overlaps
        lines.push("30-84,31-85");  // overlaps
        lines.push("2-98,64-97");   // overlaps
        lines.push("75-75,15-76");  // overlaps
        lines.push("81-81,22-81");  // overlaps
        lines.push("29-92,30-30");  // overlaps
        lines.push("95-95,14-95");  // overlaps

        assert_eq!(overlapping(&lines), 9)
    }
}