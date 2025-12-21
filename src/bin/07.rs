use std::{collections::HashMap, ops::Index};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let linecount = lines.clone().count();
    let start_x = lines.clone().next()?.find('S');

    let board: Vec<&str> = lines.clone().collect();

    let mut cache: Vec<(u64, u64)> = Vec::new();
    let tachyons = cast_tachyon(start_x?, 0, &board, linecount, &mut cache);

    Some(tachyons.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let linecount = lines.clone().count();
    let start_x = lines.clone().next()?.find('S');

    let board: Vec<&str> = lines.clone().collect();

    let mut cache: Vec<(u64, u64)> = Vec::new();
    let mut cache_data: Vec<u64> = Vec::new();
    let tachyons = cast_tachyon_2(start_x?, 0, &board, linecount, &mut cache, &mut cache_data);

    Some(tachyons)
}

fn cast_tachyon(
    x: usize,
    mut y: usize,
    board: &Vec<&str>,
    linecount: usize,
    mut cache: &mut Vec<(u64, u64)>,
) -> Vec<(u64, u64)> {
    let mut splitpoints: Vec<(u64, u64)> = Vec::new();

    while y < linecount - 1 {
        if board[y].as_bytes()[x] != '^' as u8 {
            y += 1;
        } else {
            if cache.contains(&(x as u64, y as u64)) {
                return splitpoints;
            }

            splitpoints = cast_tachyon(x - 1, y + 1, &board, linecount, &mut cache);
            let right = cast_tachyon(x + 1, y + 1, &board, linecount, &mut cache);

            for split in right {
                if !splitpoints.contains(&split) {
                    splitpoints.push(split);
                }
            }

            let point = (x as u64, y as u64);
            splitpoints.push(point);
            cache.push(point);
            break;
        }
    }

    splitpoints
}

fn cast_tachyon_2(
    x: usize,
    mut y: usize,
    board: &Vec<&str>,
    linecount: usize,
    mut cache_key: &mut Vec<(u64, u64)>,
    mut cache_data: &mut Vec<u64>,
) -> u64 {
    let mut result = 0;
    let point = (x as u64, y as u64);

    while y < linecount - 1 {
        if board[y].as_bytes()[x] != '^' as u8 {
            y += 1;
            continue;
        }

        let index = cache_key.iter().position(|&var| var == point);
        if index != None {
            let index = index.expect("This is a major error");
            return cache_data[index];
        }

        let left = cast_tachyon_2(
            x - 1,
            y + 1,
            &board,
            linecount,
            &mut cache_key,
            &mut cache_data,
        );
        let right = cast_tachyon_2(
            x + 1,
            y + 1,
            &board,
            linecount,
            &mut cache_key,
            &mut cache_data,
        );

        result = left + right;
        cache_key.push(point);
        cache_data.push(result);
        break;
    }

    if y == linecount - 1 {
        result = 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
