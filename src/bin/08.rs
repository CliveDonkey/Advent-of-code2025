advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut junctions: Vec<(i64, i64, i64)> = Vec::new();

    for line in input.lines() {
        let mut nums = line.splitn(3, ",");
        let x: i64 = nums
            .next()
            .expect("String could not be diveded in three")
            .trim()
            .parse()
            .expect("Could not convert string to int");
        let y: i64 = nums
            .next()
            .expect("String could not be diveded in three")
            .trim()
            .parse()
            .expect("Could not convert string to int");
        let z: i64 = nums
            .next()
            .expect("String could not be diveded in three")
            .trim()
            .parse()
            .expect("Could not convert string to int");

        junctions.push((x, y, z));
    }

    //find all pairs of junctions, and sort by distance:
    //this gonna take some time, 500,500.0 pairs of junctions
    let mut pairs: Vec<(f64, (i64, i64, i64), (i64, i64, i64))> = Vec::new();
    for (i, junction1) in junctions.iter().enumerate() {
        for junction2 in junctions[i + 1..].iter() {
            let difx = junction1.0 - junction2.0;
            let dify = junction1.1 - junction2.1;
            let difz = junction1.2 - junction2.2;
            let distance = f64::sqrt((difx * difx + dify * dify + difz * difz) as f64);

            pairs.push((distance, *junction1, *junction2));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let mut pair_num = 0;
    #[cfg(not(test))]
    {
        pair_num = 1000;
    }

    #[cfg(test)]
    {
        pair_num = 10;
    }
    let pair_num = pair_num;

    for pair in pairs.as_slice()[0..pair_num].iter() {
        //find circuit of both indexies in pair
        let mut circuit_index1 = None;
        let mut circuit_index2 = None;
        'search: for (i, circuit) in circuits.iter().enumerate() {
            for junction in circuit {
                if pair.1 == *junction {
                    circuit_index1 = Some(i);
                } else if pair.2 == *junction {
                    circuit_index2 = Some(i);
                }
                if circuit_index1 != None && circuit_index2 != None {
                    break 'search;
                }
            }
        }
        //add pair in circuit in apropriate manner:
        if circuit_index1 == None && circuit_index2 == None {
            //create an new circuit
            let mut circuit: Vec<(i64, i64, i64)> = Vec::new();
            circuit.push(pair.1);
            circuit.push(pair.2);
            circuits.push(circuit);
        } else if circuit_index1 != None && circuit_index2 == None {
            //add 2 to circiut of 1
            let circuit_index1 = circuit_index1.expect("");
            circuits[circuit_index1].push(pair.2);
        } else if circuit_index1 == None && circuit_index2 != None {
            //add 1 to circiut of 2
            let circuit_index2 = circuit_index2.expect("");
            circuits[circuit_index2].push(pair.1);
        } else if circuit_index1 == circuit_index2 {
            //they are in the same circuit, do nothing
        } else {
            //both are in different circuits, circuits should be merged
            let circuit_index1 = circuit_index1.expect("");
            let circuit_index2 = circuit_index2.expect("");
            for junction in circuits[circuit_index2].clone() {
                circuits[circuit_index1].push(junction);
            }
            circuits.remove(circuit_index2);
        }
    }

    circuits.sort_by(|a, b| (b.len().cmp(&a.len())));

    let result = if circuits.len() > 2 {
        circuits[0].len() * circuits[1].len() * circuits[2].len()
    } else {
        0
    };

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut junctions: Vec<(i64, i64, i64)> = Vec::new();

    for line in input.lines() {
        let mut nums = line.splitn(3, ",");
        let x: i64 = nums
            .next()
            .expect("String could not be diveded in three")
            .trim()
            .parse()
            .expect("Could not convert string to int");
        let y: i64 = nums
            .next()
            .expect("String could not be diveded in three")
            .trim()
            .parse()
            .expect("Could not convert string to int");
        let z: i64 = nums
            .next()
            .expect("String could not be diveded in three")
            .trim()
            .parse()
            .expect("Could not convert string to int");

        junctions.push((x, y, z));
    }

    //find all pairs of junctions, and sort by distance:
    //this gonna take some time, 500,500.0 pairs of junctions
    let mut pairs: Vec<(f64, (i64, i64, i64), (i64, i64, i64))> = Vec::new();
    for (i, junction1) in junctions.iter().enumerate() {
        for junction2 in junctions[i + 1..].iter() {
            let difx = junction1.0 - junction2.0;
            let dify = junction1.1 - junction2.1;
            let difz = junction1.2 - junction2.2;
            let distance = f64::sqrt((difx * difx + dify * dify + difz * difz) as f64);

            pairs.push((distance, *junction1, *junction2));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let mut pair_num = 0;
    #[cfg(not(test))]
    {
        pair_num = 1000;
    }

    #[cfg(test)]
    {
        pair_num = 10;
    }
    let pair_num = pair_num;

    for pair in pairs.as_slice()[0..pair_num].iter() {
        //find circuit of both indexies in pair
        let mut circuit_index1 = None;
        let mut circuit_index2 = None;
        'search: for (i, circuit) in circuits.iter().enumerate() {
            for junction in circuit {
                if pair.1 == *junction {
                    circuit_index1 = Some(i);
                } else if pair.2 == *junction {
                    circuit_index2 = Some(i);
                }
                if circuit_index1 != None && circuit_index2 != None {
                    break 'search;
                }
            }
        }
        //add pair in circuit in apropriate manner:
        if circuit_index1.is_none() && circuit_index2.is_none() {
            //create an new circuit
            let mut circuit: Vec<(i64, i64, i64)> = Vec::new();
            circuit.push(pair.1);
            circuit.push(pair.2);
            circuits.push(circuit);
        } else if circuit_index1.is_some() && circuit_index2.is_none() {
            //add 2 to circiut of 1
            let circuit_index1 = circuit_index1.expect("");
            circuits[circuit_index1].push(pair.2);
        } else if circuit_index1 == None && circuit_index2 != None {
            //add 1 to circiut of 2
            let circuit_index2 = circuit_index2.expect("");
            circuits[circuit_index2].push(pair.1);
        } else if circuit_index1 == circuit_index2 {
            //they are in the same circuit, do nothing
        } else {
            //both are in different circuits, circuits should be merged
            let circuit_index1 = circuit_index1.expect("");
            let circuit_index2 = circuit_index2.expect("");
            for junction in circuits[circuit_index2].clone() {
                circuits[circuit_index1].push(junction);
            }
            circuits.remove(circuit_index2);
        }
    }

    let mut i: usize = pair_num;
    let mut junc_in_circuit = 0;
    for circuit in &circuits {
        junc_in_circuit += circuit.len();
    }

    while circuits.len() > 1 || junc_in_circuit < junctions.len() {
        let pair = pairs[i];
        //find circuit of both indexies in pair
        let mut circuit_index1 = None;
        let mut circuit_index2 = None;
        'search: for (i, circuit) in circuits.iter().enumerate() {
            for junction in circuit {
                if pair.1 == *junction {
                    circuit_index1 = Some(i);
                } else if pair.2 == *junction {
                    circuit_index2 = Some(i);
                }
                if circuit_index1 != None && circuit_index2 != None {
                    break 'search;
                }
            }
        }
        //add pair in circuit in apropriate manner:
        if circuit_index1 == None && circuit_index2 == None {
            //create an new circuit
            let mut circuit: Vec<(i64, i64, i64)> = Vec::new();
            circuit.push(pair.1);
            circuit.push(pair.2);
            circuits.push(circuit);
        } else if circuit_index1 != None && circuit_index2 == None {
            //add 2 to circiut of 1
            let circuit_index1 = circuit_index1.expect("");
            circuits[circuit_index1].push(pair.2);
        } else if circuit_index1 == None && circuit_index2 != None {
            //add 1 to circiut of 2
            let circuit_index2 = circuit_index2.expect("");
            circuits[circuit_index2].push(pair.1);
        } else if circuit_index1 == circuit_index2 {
            //they are in the same circuit, do nothing
        } else {
            //both are in different circuits, circuits should be merged
            let circuit_index1 = circuit_index1.expect("");
            let circuit_index2 = circuit_index2.expect("");
            for junction in circuits[circuit_index2].clone() {
                circuits[circuit_index1].push(junction);
            }
            circuits.remove(circuit_index2);
        }

        junc_in_circuit = 0;
        for circuit in &circuits {
            junc_in_circuit += circuit.len();
        }
        i += 1;
    }

    let pair = pairs[i - 1];
    let result = pair.1.0 * pair.2.0;

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
