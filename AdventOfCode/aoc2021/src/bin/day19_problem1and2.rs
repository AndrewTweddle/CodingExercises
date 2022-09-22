use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const MIN_MATCHING_BEACONS: usize = 12;

type Coordinates = (i64, i64, i64);
type Scanner = Vec<Coordinates>;

fn main() {
    let start_time = Instant::now();

    let contents = fs::read_to_string("data/day19_input.txt").unwrap();
    let mut scanners: Vec<Scanner> = contents
        .split("\n\n")
        .map(|scanner_str| {
            scanner_str
                .lines()
                .skip(1)
                .map(|scanner_line| {
                    let (x_str, yz_str) = scanner_line.split_once(',').unwrap();
                    let (y_str, z_str) = yz_str.split_once(',').unwrap();
                    (
                        x_str.parse::<i64>().unwrap(),
                        y_str.parse::<i64>().unwrap(),
                        z_str.parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<Coordinates>>()
        })
        .collect();

    // Arbitrarily choose the first scanner's coordinates as the origin of the coordinate system
    let known_scanner = scanners.remove(0);
    let (scanner_locations, beacons, unmatched_scanners) =
        get_scanner_locations_beacons_and_unmatched_scanners(known_scanner, &scanners);

    if !unmatched_scanners.is_empty() {
        println!(
            "Error: {} scanners could not be matched!",
            unmatched_scanners.len()
        );
        println!("Unmatched scanners: {:?}", unmatched_scanners);
        println!();
    }
    println!("Part 1: # of beacons found: {}", beacons.len());

    let max_manhattan_distance = scanner_locations
        .iter()
        .enumerate()
        .filter_map(|(i, beacon1)| {
            scanner_locations[(i + 1)..]
                .iter()
                .map(|beacon2| manhattan_distance(beacon1, beacon2))
                .max()
        })
        .max()
        .unwrap();

    println!(
        "Part 2: max Manhattan distance between any 2 scanners = {}",
        max_manhattan_distance
    );
    println!();

    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);
}

fn get_scanner_locations_beacons_and_unmatched_scanners(
    known_scanner: Scanner,
    scanners: &[Scanner]
) -> (Vec<Coordinates>, HashSet<Coordinates>, Vec<Scanner>) {
    let mut unmatched_scanners = scanners.to_vec();

    // Track unique beacons
    let mut beacons: HashSet<Coordinates> = HashSet::new();

    // Track the positions of the scanners (once located by matching up beacons of other scanners)
    let mut scanner_locations: Vec<Coordinates> = Vec::new();
    scanner_locations.push((0, 0, 0));

    // Track scanners which have been successfully oriented and translated into a known position
    let mut located_scanners = Vec::<Scanner>::with_capacity(unmatched_scanners.len());
    located_scanners.push(known_scanner);

    while let Some(scanner_to_match) = located_scanners.pop() {
        // Record all beacons from this located scanner
        beacons.extend(scanner_to_match.iter());

        // Find all un-oriented scanners which can be reoriented and translated
        // so that a sufficient number of beacons overlaps with the scanner to match
        let mut i: usize = 0;
        while i < unmatched_scanners.len() {
            let candidate = &unmatched_scanners[i];
            if let Some((scanner_pos, transformed_beacons)) =
                get_overlapping_scanner(&scanner_to_match, candidate)
            {
                located_scanners.push(transformed_beacons);
                scanner_locations.push(scanner_pos);
                unmatched_scanners.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    (scanner_locations, beacons, unmatched_scanners)
}

fn get_overlapping_scanner(
    oriented_scanner: &Scanner,
    other_scanner: &Scanner,
) -> Option<(Coordinates, Scanner)> {
    for transform_id in 0..24 {
        let reoriented: Vec<Coordinates> = other_scanner
            .iter()
            .map(|coords| rotate(coords, transform_id))
            .collect();
        for known_beacon in oriented_scanner {
            // Try each of the reoriented scanners' beacons to see if it is the same beacon
            for candidate_coord in &reoriented {
                let scanner_pos = diff_of_coords(known_beacon, candidate_coord);

                // It's a match if enough other reoriented beacons
                // also match any of the known beacons
                let match_count = reoriented
                    .iter()
                    .filter(|other_beacon| {
                        let other_beacon_translated = sum_of_coords(other_beacon, &scanner_pos);
                        oriented_scanner
                            .iter()
                            .any(|&oriented_beacon| oriented_beacon == other_beacon_translated)
                    })
                    .count();
                if match_count >= MIN_MATCHING_BEACONS {
                    let transformed_scanner: Scanner = reoriented
                        .iter()
                        .map(|beacon| sum_of_coords(beacon, &scanner_pos))
                        .collect();
                    return Some((scanner_pos, transformed_scanner));
                }
            }
        }
    }
    None
}

fn manhattan_distance(a: &Coordinates, b: &Coordinates) -> i64 {
    let diff = diff_of_coords(a, b);
    diff.0.abs() + diff.1.abs() + diff.2.abs()
}

fn sum_of_coords(a: &Coordinates, b: &Coordinates) -> Coordinates {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn diff_of_coords(a: &Coordinates, b: &Coordinates) -> Coordinates {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn rotate(coords: &Coordinates, rotation_id: u8) -> Coordinates {
    let (mut x, mut y, mut z) = coords;
    let mut num_signs_flipped = 0;

    if rotation_id & 1 != 0 {
        x = -x;
        num_signs_flipped += 1;
    }

    if rotation_id & 2 != 0 {
        y = -y;
        num_signs_flipped += 1;
    }

    if rotation_id & 4 != 0 {
        z = -z;
        num_signs_flipped += 1;
    }

    if num_signs_flipped % 2 == 1 {
        // The parity has been flipped, so flip it back by reversing the order of the axes:
        (x, z) = (z, x);
    }

    let cycle_count = (rotation_id / 8) % 3;
    match cycle_count {
        1 => (x, y, z) = (z, x, y),
        2 => (x, y, z) = (y, z, x),
        _ => {}
    }

    (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rotation_tests {
        use super::{rotate, Coordinates};
        use std::collections::HashSet;

        #[test]
        fn count_all_rotations() {
            let rotations: HashSet<Coordinates> = (0..24)
                .map(|rotation_id| rotate(&(1, 2, 3), rotation_id))
                .collect();
            assert_eq!(rotations.len(), 24);
        }

        #[test]
        fn mirror_images_are_excluded() {
            let rotations: HashSet<Coordinates> = (0..24)
                .map(|rotation_id| rotate(&(1, 2, 3), rotation_id))
                .collect();
            let mirror_rotations: HashSet<Coordinates> = rotations
                .iter()
                .flat_map(|rotation| {
                    [
                        (-rotation.0, rotation.1, rotation.2),
                        (rotation.0, -rotation.1, rotation.2),
                        (rotation.0, rotation.1, -rotation.2),
                        (-rotation.0, -rotation.1, -rotation.2),
                    ]
                })
                .collect();

            assert_eq!(mirror_rotations.len(), 24);
            assert_eq!(rotations.intersection(&mirror_rotations).count(), 0)
        }
    }

    mod manhattan_distance_tests {
        use super::manhattan_distance;

        #[test]
        fn test_manhattan_distance() {
            let beacon1 = (1105, -1205, 1229);
            let beacon2 = (-92, -2380, -20);
            assert_eq!(manhattan_distance(&beacon1, &beacon2), 3621);
        }
    }
}
