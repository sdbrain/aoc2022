use std::{collections::BinaryHeap, vec};

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Day1 {}
    }

    pub fn which_elve_max_calory(&self, f_string: &str) -> u32 {
        let lines: Vec<&str> = f_string.split_terminator("\n").collect();
        let mut max = 0;
        let mut sum = 0;
        for line in lines {
            if line.is_empty() {
                if sum > max {
                    max = sum;
                }
                sum = 0;
                continue;
            }
            let num: u32 = line.trim().parse().unwrap();
            sum += num;
        }

        max
    }

    pub fn top_3_elves_v1(&self, f_string: &str) -> u32 {
        let lines: Vec<&str> = f_string.split_terminator("\n").collect();

        let mut max = vec![0, 0, 0];
        let mut sum: u32 = 0;
        for line in lines {
            if line.is_empty() {
                let result = max.iter().position(|x| sum > *x);
                match result {
                    Some(idx) => {
                        max[idx] = sum;
                    }
                    None => {}
                };

                max.sort();
                sum = 0;
                continue;
            }
            let num: u32 = line.trim().parse().unwrap();
            sum += num;
        }
        max.iter().sum()
    }

    // comparison code for reference.. BinaryHeap based
    // https://github.com/kprav33n/aoc22.rs/blob/main/src/day01.rs#L18
    pub fn top_3_elves_v2(&self, f_string: &str) -> i64 {
        let mut cals: BinaryHeap<i64> = f_string
            .trim()
            .split("\n\n")
            .map(|elf| {
                elf.trim()
                    .split('\n')
                    .filter_map(|item| item.trim().parse::<i64>().ok())
                    .sum()
            })
            .collect();
        (0..3).filter_map(|_| cals.pop()).sum()
    }
}

#[cfg(test)]
mod test_day1 {
    use super::*;

    #[test]
    fn test_which_elve_max_calory() {
        let input = r###"
        1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"###;
        let day1 = Day1::new();
        let sum = day1.which_elve_max_calory(input);
        assert_eq!(sum, 24000);
    }
    #[test]
    fn test_top_3_elves_v1() {
        let input = r###"
        1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"###;
        let day1 = Day1::new();
        let sum = day1.top_3_elves_v1(input);
        assert_eq!(sum, 41000);
    }
    #[test]
    fn test_top_3_elves_v2() {
        let input = r###"
        1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"###;
        let day1 = Day1::new();
        let sum = day1.top_3_elves_v2(input);
        assert_eq!(sum, 41000);
    }
}
