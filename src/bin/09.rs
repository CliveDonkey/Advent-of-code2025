use std::cmp::Ordering;

use num_traits::abs_sub;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Line {
    start: (u64, u64),
    end: (u64, u64),
    inside: Direction,
}

#[derive(Debug)]
struct Area {
    corner1: (u64, u64),
    corner2: (u64, u64),
    size: u64,
}

impl Ord for Area {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl Eq for Area {}

fn test_line(area: &Area, line: &Line) -> bool {
    let x_min = area.corner1.0.min(area.corner2.0);
    let x_max = area.corner1.0.max(area.corner2.0);
    let y_min = area.corner1.1.min(area.corner2.1);
    let y_max = area.corner1.1.max(area.corner2.1);

    //Check if line intersects area border:
    let not_intersects = match line.inside {
        Direction::Right => {
            line.start.0 < x_min
                || x_max < line.start.0
                || line.start.1 < y_min
                || y_max < line.end.1
        }
        Direction::Down => {
            line.end.0 < x_min
                || x_max < line.start.0
                || line.start.1 < y_min
                || y_max < line.start.1
        }
        Direction::Left => {
            line.start.0 < x_min
                || x_max < line.start.0
                || line.end.1 < y_min
                || y_max < line.start.1
        }
        Direction::Up => {
            line.start.0 < x_min
                || x_max < line.end.0
                || line.start.1 < y_min
                || y_max < line.start.1
        }
    };
    if not_intersects {
        return true;
    }

    //check if either start or end is inside shape
    let start_end = x_min < line.start.0
        && line.start.0 < x_max
        && y_min < line.start.1
        && line.start.1 < y_max
        || x_min < line.end.0 && line.end.0 < x_max && y_min < line.end.1 && line.end.1 < y_max;
    if start_end {
        return false;
    }
    //check if line goes across entire area:
    //this match is fucked, .inside points to the right relative to line direction...
    let across = match line.inside {
        Direction::Up => {
            y_min < line.start.1
                && line.start.1 < y_max
                && line.end.0 < y_min
                && y_max < line.start.0
        }
        Direction::Down => {
            y_min < line.start.1
                && line.start.1 < y_max
                && line.start.0 < y_min
                && y_max < line.end.0
        }
        Direction::Left => {
            x_min < line.start.0
                && line.start.0 < x_max
                && line.start.1 < x_min
                && x_max < line.end.1
        }
        Direction::Right => {
            x_min < line.start.0
                && line.start.0 < x_max
                && line.end.1 < x_min
                && x_max < line.start.1
        }
    };
    if across {
        return false;
    }

    //if only touches area-border with start/end, skip:
    let corner_border = match line.inside {
        Direction::Right => line.start.1 == y_min || line.end.1 == y_max,
        Direction::Down => line.start.0 == x_min || line.end.0 == x_max,
        Direction::Left => line.end.1 == y_min || line.start.1 == y_max,
        Direction::Up => line.end.0 == x_min || line.start.0 == x_max,
    };
    if corner_border {
        return true;
    }

    //only option left should be that line lies along the border, so we check that
    //we are on the correct side of the line:
    //this match is fucked, .inside points to the right relative to line direction...
    let inside_line = match line.inside {
        Direction::Up => area.corner1.1.max(area.corner2.1) <= line.start.1,
        Direction::Right => area.corner1.0.min(area.corner2.0) >= line.start.0,
        Direction::Down => area.corner1.1.min(area.corner2.1) >= line.start.1,
        Direction::Left => area.corner1.0.max(area.corner2.0) <= line.start.0,
    };

    if !inside_line {
        return false;
    }
    true
}

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut corners: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        let mut nums = line.split(",");
        let x = nums.next()?;
        let y = nums.next()?;
        let x: u64 = x.trim().parse().expect("Failed to convert {x} to int");
        let y: u64 = y.trim().parse().expect("Failed to convert {y} to int");

        corners.push((x, y));
    }

    let mut areas: Vec<u64> = Vec::new();
    for (i, corner1) in corners.iter().enumerate() {
        for corner2 in &corners[i + 1..] {
            let (x1, y1) = corner1;
            let (x2, y2) = corner2;
            areas.push((x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1));
        }
    }

    areas.sort();

    Some(areas.pop()?)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut corners: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        let mut nums = line.split(",");
        let x = nums.next()?;
        let y = nums.next()?;
        let x: u64 = x.trim().parse().expect("Failed to convert {x} to int");
        let y: u64 = y.trim().parse().expect("Failed to convert {y} to int");

        corners.push((x, y));
    }

    let mut lines: Vec<Line> = Vec::new();
    let mut areas: Vec<Area> = Vec::new();

    for (i, corner1) in corners.iter().enumerate() {
        //Constructing all lines that make up border
        let next_corner = corners[(i + 1) % corners.len()];

        let line = Line {
            start: *corner1,
            end: next_corner,
            inside: {
                if corner1.0 == next_corner.0 {
                    //line is vertical
                    if corner1.1 < next_corner.1 {
                        //line goes down => inside is left
                        Direction::Left
                    } else {
                        //line goes up => inside is right
                        Direction::Right
                    }
                } else {
                    //line is horizontal
                    if corner1.0 < next_corner.0 {
                        //line goes to the right => inside is down
                        Direction::Down
                    } else {
                        //line goes to the left => inside is up
                        Direction::Up
                    }
                }
            },
        };
        lines.push(line);

        //constructing all possible squares
        for corner2 in &corners[i + 1..] {
            let (x1, y1) = corner1;
            let (x2, y2) = corner2;
            let size = (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1);

            let area = Area {
                corner1: *corner1,
                corner2: *corner2,
                size: size,
            };

            areas.push(area);
        }
    }

    areas.sort();

    'outer: loop {
        let area = areas.pop();
        if area == None {
            break;
        }
        let area = area?;

        for line in &lines {
            //line-check logic:
            //if line and area does not overlap at all
            //area is either completly insde or completly outside
            //this is not possible for all lines, because:
            //corners of area is part of lines.
            //first skip all lines that does not intersect
            //then, we only need to check following:
            //does line cross area border (bad)
            //if not, is area on right side of line

            if test_line(&area, line) {
                continue;
            } else {
                continue 'outer;
            }
        }
        //hiting this point means we are inside all lines
        return Some(area.size);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
