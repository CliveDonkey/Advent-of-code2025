advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zero_count = 0;

    let lines = input.split("\n");
    for line in lines {
        let (direction, increment) = line.split_at(1);
        let increment: i64 = match increment.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        match direction {
            //offseting by 100 to fix negative remainder
            "L" => dial = (dial - increment - 100) % 100,
            "R" => dial = (dial + increment) % 100,
            _ => {
                println!("Critical structrure coruption, exiting..");
                break;
            }
        };

        if dial == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zero_count = 0;

    let lines = input.split("\n");
    for line in lines {
        let (direction, increment) = line.split_at(1);
        let mut increment: i64 = match increment.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        //counting all whole rotations as passing zero
        while increment > 99 {
            increment -= 100;
            zero_count += 1;
        }
        while increment < 0 {
            increment += 100;
            zero_count += 1;
        }

        let prev = dial;
        match direction {
            "L" => {
                //offseting by 100 to fix negative remainder
                dial = (dial - increment + 100) % 100;
                if dial > prev && prev != 0 {
                    zero_count += 1;
                }
            }
            "R" => {
                dial = (dial + increment) % 100;
                if dial < prev && dial != 0 {
                    zero_count += 1;
                }
            }
            _ => {
                println!("Critical structrure coruption, exiting..");
                break;
            }
        };

        if dial == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
