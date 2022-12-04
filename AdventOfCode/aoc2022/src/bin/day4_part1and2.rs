type Assignment = (u32, u32);
type PairPredicate = fn(Assignment, Assignment) -> bool;

fn main() {
    let contents = std::fs::read_to_string("data/day4_input.txt").unwrap();
    let part1 = count_matching_lines(&contents, either_assignment_contains_the_other);
    let part2 = count_matching_lines(&contents, assignments_overlap);
    println!("2022 day  part 1 answer: {}", part1);
    println!("2022 day  part 2 answer: {}", part2);
}

fn count_matching_lines(contents: &str, predicate: PairPredicate) -> usize {
    contents
        .lines()
        .filter(|ln| {
            let (l_str, r_str) = ln.split_once(',').unwrap();
            let (lft, rgt) = (parse_assignment(l_str), parse_assignment(r_str));
            predicate(lft, rgt)
        })
        .count()
}

fn parse_assignment(elf_str: &str) -> Assignment {
    let (start_str, end_str) = elf_str.split_once('-').unwrap();
    let start = start_str.parse().unwrap();
    let end = end_str.parse().unwrap();
    (start, end)
}

fn either_assignment_contains_the_other(lft: Assignment, rgt: Assignment) -> bool {
    (lft.0 <= rgt.0 && lft.1 >= rgt.1) || (lft.0 >= rgt.0 && lft.1 <= rgt.1)
}

fn assignments_overlap(lft: Assignment, rgt: Assignment) -> bool {
    lft.0 <= rgt.1 && rgt.0 <= lft.1
}

#[cfg(test)]
mod tests {
    mod part1_tests {
        use crate::either_assignment_contains_the_other;

        #[test]
        fn test_first_fully_contains_second() {
            assert!(either_assignment_contains_the_other((2, 8), (3, 7)));
        }

        #[test]
        fn test_second_fully_contains_first() {
            assert!(either_assignment_contains_the_other((3, 7), (2, 8)));
        }

        #[test]
        fn test_first_contains_second_matching_on_the_right() {
            assert!(either_assignment_contains_the_other((4, 6), (5, 6)));
        }

        #[test]
        fn test_second_contains_first_matching_on_the_right() {
            assert!(either_assignment_contains_the_other((5, 6), (4, 6)));
        }

        #[test]
        fn test_first_contains_second_matching_on_the_left() {
            assert!(either_assignment_contains_the_other((4, 6), (4, 5)));
        }

        #[test]
        fn test_second_contains_first_matching_on_the_left() {
            assert!(either_assignment_contains_the_other((4, 5), (4, 6)));
        }

        #[test]
        fn test_matching_assignments() {
            assert!(either_assignment_contains_the_other((3, 7), (3, 7)));
        }

        #[test]
        fn test_neither_contains_the_other() {
            assert!(!either_assignment_contains_the_other((5, 7), (7, 9)));
        }
    }

    mod part2_tests {
        use crate::assignments_overlap;

        #[test]
        fn test_overlap_in_single_section() {
            // 5-7,7-9 overlaps in a single section, 7.
            assert!(assignments_overlap((5, 7), (7, 9)))
        }

        #[test]
        fn test_overlap_when_fully_containing_other_assignment() {
            // 2-8,3-7 overlaps all of the sections 3 through 7.
            assert!(assignments_overlap((2, 8), (3, 7)))
        }

        #[test]
        fn test_overlap_in_a_single_section_overlapping_assignment() {
            // 6-6,4-6 overlaps in a single section, 6.
            assert!(assignments_overlap((6, 6), (4, 6)))
        }

        #[test]
        fn test_overlap_in_multiple_sections() {
            // 2-6,4-8 overlaps in sections 4, 5, and 6.
            assert!(assignments_overlap((2, 6), (4, 8)))
        }

        #[test]
        fn test_no_overlap_with_first_assignment_earlier() {
            assert!(!assignments_overlap((2, 4), (6, 8)))
        }

        #[test]
        fn test_no_overlap_with_first_assignment_later() {
            assert!(!assignments_overlap((6, 8), (1, 3)))
        }
    }
}
