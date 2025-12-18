advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mult,
    INVALID,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ops: Vec<Operator> = Vec::new();

    let mut lines = input.lines().rev();

    let last_line = lines.next();
    for operator in last_line?.split_ascii_whitespace() {
        ops.push(match operator.trim() {
            "+" => Operator::Add,
            "*" => Operator::Mult,
            _ => Operator::INVALID,
        });
    }

    let width = ops.len();
    let height = lines.clone().count();

    //creating column-major grid of numbers
    //order matters because cache-locality
    let mut numbers = vec![vec![0; height]; width];

    for (y, line) in lines.enumerate() {
        let nums = line.split_ascii_whitespace();
        for (x, num) in nums.enumerate() {
            numbers[x][y] = num.trim().parse().expect("Could not convert {num} to int");
        }
    }

    let mut total = 0;
    for (i, column) in numbers.iter().enumerate() {
        let mut col_total = column[0];
        for num in &column[1..] {
            col_total = match ops[i] {
                Operator::Add => col_total + num,
                Operator::Mult => col_total * num,
                Operator::INVALID => 0,
            }
        }
        total += col_total;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let first_line = lines.clone().next();

    let mut ops: Vec<Operator> =
        vec![Operator::INVALID; first_line?.split_ascii_whitespace().count()];

    let grid_width = first_line?.len();
    let grid_height = lines.clone().count();

    let grid: Vec<_> = input.lines().collect();

    //creating column-major grid of numbers
    let width = ops.len();
    let height = lines.clone().count() - 1;

    let mut numbers = vec![vec![0; height]; width];

    //for each column from top to bottom we should create the numbers
    //if we go from right to left and top to bottom, an operator, eg +
    //signifies end of number, and we can move on
    //top to bottom also lets us do number*10+char to build numbers
    let mut col_index = width - 1;
    let mut row_index = 0;

    'outer: for x in (0..grid_width).rev() {
        let mut number: String = String::new();

        for y in 0..grid_height {
            //parse operator at bottom line
            if y == grid_height - 1 {
                if grid[y].as_bytes()[x] == '+' as u8 {
                    println!("number: {number}, row_index: {row_index}");
                    ops[col_index] = Operator::Add;
                    numbers[col_index][row_index] = number
                        .trim()
                        .parse()
                        .expect("Could not convert {number} to int");
                    row_index = 0;
                    if col_index == 0 {
                        break 'outer;
                    }
                    col_index -= 1;
                    continue 'outer;
                } else if grid[y].as_bytes()[x] == '*' as u8 {
                    println!("number: {number}, row_index: {row_index}");
                    ops[col_index] = Operator::Mult;
                    numbers[col_index][row_index] = number
                        .trim()
                        .parse()
                        .expect("Could not convert {number} to int");
                    row_index = 0;
                    if col_index == 0 {
                        break 'outer;
                    }
                    col_index -= 1;
                    continue 'outer;
                }
                continue;
            }
            if grid[y].as_bytes()[x] == ' ' as u8 {
                continue;
            } else {
                number.push(grid[y].as_bytes()[x] as char);
            }
        }
        println!("number: {number}, row_index: {row_index}");
        if number.is_empty() {
            continue 'outer;
        }
        numbers[col_index][row_index] = number
            .trim()
            .parse()
            .expect("Could not convert {number} to int");
        row_index += 1;
    }

    //transposed debug print:
    println!("Numbers:");
    for col in &numbers {
        for num in col {
            print!("{num} ");
        }
        println!();
    }

    let mut total = 0;
    for (i, column) in numbers.iter().enumerate() {
        let mut col_total = column[0];
        for num in &column[1..] {
            col_total = match ops[i] {
                Operator::Add => col_total + num,
                Operator::Mult => col_total * { if *num == 0 { 1 } else { *num } },
                Operator::INVALID => 0,
            }
        }
        total += col_total;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
