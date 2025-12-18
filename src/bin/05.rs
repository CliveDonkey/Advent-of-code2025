use std::cmp::Ordering;

advent_of_code::solution!(5);

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}
impl Eq for Range {}

pub fn part_one(input: &str) -> Option<u64> {
    //split input by empty line between ranges and ingredients
    let (ranges, ingredients) = input.split_once("\n\n")?;

    //load input into ranges
    let mut range_list: Vec<Range> = Vec::new();

    for range in ranges.lines() {
        let (start, end) = range.split_once('-')?;
        let r = Range {
            start: start
                .trim()
                .parse()
                .expect("Could not convert {start} to int"),
            end: end.trim().parse().expect("Could not convert {end} to int"),
        };

        range_list.push(r);
    }

    //same parsing for ingredients:
    let mut ingredient_list: Vec<u64> = Vec::new();

    for ingredient in ingredients.lines() {
        ingredient_list.push(
            ingredient
                .trim()
                .parse()
                .expect("Could not convert {ingredient} to int"),
        );
    }

    let mut fresh_count = 0;

    for ing in ingredient_list {
        for range in range_list.as_slice() {
            if range.start <= ing && ing <= range.end {
                fresh_count += 1;
                break;
            }
        }
    }

    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    //split input by empty line between ranges and ingredients
    let (ranges, _) = input.split_once("\n\n")?;

    //load input into ranges
    let mut range_list: Vec<Range> = Vec::new();

    for range in ranges.lines() {
        let (start, end) = range.split_once('-')?;
        let r = Range {
            start: start
                .trim()
                .parse()
                .expect("Could not convert {start} to int"),
            end: end.trim().parse().expect("Could not convert {end} to int"),
        };

        range_list.push(r);
    }

    //merge overlapping ranges:
    range_list.sort();
    let mut final_ranges: Vec<Range> = Vec::new();
    final_ranges.push(range_list[0]);

    'outer: for range2 in &range_list[1..] {
        'inner: for (i, range1) in final_ranges.clone().iter().enumerate() {
            if range1.end < range2.start {
                continue 'inner;
            } else if range1.end >= range2.end {
                //range2 is contained within range1:
                continue 'outer;
            } else if range1.start <= range2.start
                && range2.start <= range1.end
                && range1.end < range2.end
            {
                //ranges overlap, extend end of range1 to end of range2
                final_ranges[i].end = range2.end;
                continue 'outer;
            }
        }
        //this should only be reached if range2 is not in final_ranges
        //and does not overlap with any ranges there
        final_ranges.push(*range2);
    }

    let mut ingredient_count = 0;

    for range in &final_ranges {
        ingredient_count += range.end - range.start + 1;
    }
    Some(ingredient_count)
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
        assert_eq!(result, Some(14));
    }
}
