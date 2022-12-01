use std::fs;
use std::iter::Sum;
use std::ptr;

struct Elf {
    pub items: Vec<u64>, // every meals calorie count
}

impl Elf {
    pub fn new() -> Elf {
        Elf { items: Vec::new() }
    }

    // pub fn add_items(&mut self, items: Vec<u64>) {
    //     self.items.extend(items);
    // }

    pub fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &Vec<u64> {
        &self.items
    }

    pub fn get_total_calories(&self) -> u64 {
        self.items.iter().sum()
    }
}

fn main() {
    let file_path = "input.txt";

    // read file to string
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    // split string into lines
    let lines: Vec<&str> = contents.split("\n").collect();


    // every elf is delimited by a blank line
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf = Elf::new();

    // if a line is blank, we have reached the end of an elf's items
    for line in lines {
        if line == "" {
            elves.push(elf);
            elf = Elf::new();
        }
        // elf has one item per line
        else {
            let item = line.parse::<u64>();
            // print line if parsing has failed
            if item.is_err() {
                println!("Failed to parse line: {}", line);
            }
            // add item to elf
            else {
                elf.add_item(item.unwrap());
            }
        }
    }

    // sort the elves array by total calories carrying
    elves.sort_by(|x, y| x.get_total_calories().cmp(&y.get_total_calories()));


    // print every elf's index and total calories
    for (i, elf) in elves.iter().enumerate() {
        println!("Elf {} has {} items with a total of {} calories", i, elf.get_items().len(), elf.get_total_calories());
    }

    // sum the top 3
    let sum: u64 = elves.iter().rev().take(3).map(|e| e.get_total_calories()).sum();

    // print
    println!("\nTotal calories carried by the top 3 elves: {}", sum);
}
