use std::error::Error;

pub fn solve(input: String) -> Result<(String, String), Box<dyn Error>> {

    let ans_part1 = part1(&input);
    let ans_part2 = part2(&input);
    Ok((ans_part1, ans_part2))
}

fn part1(input: &String) -> String {
    let mut max_elf_cal = 0;
    let mut curr_elf_cal = 0;

    let item_iter = input.lines();
    for item in item_iter {
        if item.is_empty() {
            if curr_elf_cal > max_elf_cal {
                max_elf_cal = curr_elf_cal
            }
            curr_elf_cal = 0;
        } else {
            curr_elf_cal += item.parse::<u32>().unwrap();
        }
    }
    max_elf_cal.to_string()
}

fn part2(input: &String) -> String {
    let mut elf_vec: Vec<u32> = Vec::new();
    let mut curr_elf_cal = 0;

    let item_iter = input.lines();
    for item in item_iter {
        if item.is_empty() {
            elf_vec.push(curr_elf_cal);
            curr_elf_cal = 0;
        } else {
            curr_elf_cal += item.parse::<u32>().unwrap();
        }
    }
    elf_vec.sort();
    elf_vec.reverse();
    let tri_sum: u32 = elf_vec[0..3].iter().sum();

    tri_sum.to_string()
}