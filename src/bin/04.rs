use std::ops::Index;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let linelength = match input.find("\n") {
        Some(num) => num,
        None => 0,
    };

    let linecount = input.lines().count();

    let mut board: Vec<u8> = vec![];

    //reading data into board
    let lines = input.split("\n");
    for line in lines {
        for char in line.chars() {
            match char {
                '.' => board.push(0),
                '@' => board.push(1),
                '\n' => continue,
                _ => return None,
            };
        }
    }

    let mut rolls = 0;

    for y in 0..linecount {
        for x in 0..linelength {
            if check_roll(x, y, &board.as_slice(), linelength, linecount) {
                rolls += 1;
                continue;
            }
        }
    }

    Some(rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let linelength = match input.find("\n") {
        Some(num) => num,
        None => 0,
    };

    let linecount = input.lines().count();

    let mut board: Vec<u8> = vec![];

    //reading data into board
    let lines = input.split("\n");
    for line in lines {
        for char in line.chars() {
            match char {
                '.' => board.push(0),
                '@' => board.push(1),
                '\n' => continue,
                _ => return None,
            };
        }
    }

    let mut rolls = 0;
    let mut last_rolls = 1; //use any value but 0

    while last_rolls > 0 {
        last_rolls = 0;

        for y in 0..linecount {
            for x in 0..linelength {
                if check_roll(x, y, &board.as_slice(), linelength, linecount) {
                    board[y * linelength + x] = 0;
                    rolls += 1;
                    last_rolls += 1;
                    continue;
                }
            }
        }
    }

    Some(rolls)
}

//returns true if line is removable
fn check_roll(x: usize, y: usize, data: &[u8], linelength: usize, linecount: usize) -> bool {
    let x = x as i64;
    let y = y as i64;
    let linelength = linelength as i64;
    let linecount = linecount as i64;

    if data[(y * linelength + x) as usize] == 0 {
        return false;
    }

    let mut adjacent = 0;

    for y_search in 0..3 {
        //bounds check y:
        if (y_search + y - 1 < 0) | (y_search + y - 1 >= linecount) {
            continue;
        }

        for x_search in 0..3 {
            //bounds check x:
            if (x_search == 1) & (y_search == 1) {
                continue;
            }
            if (x_search + x - 1 < 0) | (x_search + x - 1 >= linelength) {
                continue;
            }

            //doing check and adding to count
            if data[((y + y_search - 1) * linelength + (x + x_search - 1)) as usize] == 1 {
                adjacent += 1;
            }
        }
    }

    adjacent < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
