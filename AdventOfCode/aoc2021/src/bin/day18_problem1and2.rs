use std::fs;
use std::ops::Add;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
struct SnailFishNumber {
    pair: (Element, Element),
}

impl SnailFishNumber {
    fn to_number_pair(&self) -> (usize, usize) {
        if let (Element::RegularNumber(num0), Element::RegularNumber(num1)) = self.pair {
            (num0, num1)
        } else {
            panic!("Snailfish pair is not a pair of numbers");
        }
    }

    fn magnitude(&self) -> usize {
        3 * self.pair.0.magnitude() + 2 * self.pair.1.magnitude()
    }
}

impl Add for SnailFishNumber {
    type Output = SnailFishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let left_num = Element::NestedSnailFishNumber(Box::new(self));
        let right_num = Element::NestedSnailFishNumber(Box::new(rhs));
        let mut sfnum_sum = SnailFishNumber {
            pair: (left_num, right_num),
        };
        sfnum_sum.reduce();
        sfnum_sum
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
    RegularNumber(usize),
    NestedSnailFishNumber(Box<SnailFishNumber>),
}

impl Element {
    fn new_number_pair(left_val: usize, right_val: usize) -> Self {
        Element::NestedSnailFishNumber(Box::new(SnailFishNumber {
            pair: (
                Element::RegularNumber(left_val),
                Element::RegularNumber(right_val),
            ),
        }))
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::RegularNumber(reg_num) => *reg_num,
            Element::NestedSnailFishNumber(boxed_sf_num) => boxed_sf_num.magnitude(),
        }
    }

    fn try_add_to_leftmost_value_in_branch(&mut self, value_to_add: usize) -> bool {
        match self {
            Element::NestedSnailFishNumber(boxed_sfnum) => {
                if boxed_sfnum
                    .pair
                    .0
                    .try_add_to_leftmost_value_in_branch(value_to_add)
                {
                    true
                } else {
                    boxed_sfnum
                        .pair
                        .1
                        .try_add_to_leftmost_value_in_branch(value_to_add)
                }
            }
            Element::RegularNumber(curr_value) => {
                *curr_value += value_to_add;
                true
            }
        }
    }

    fn try_add_to_rightmost_value_in_branch(&mut self, value_to_add: usize) -> bool {
        match self {
            Element::NestedSnailFishNumber(boxed_sfnum) => {
                if boxed_sfnum
                    .pair
                    .1
                    .try_add_to_rightmost_value_in_branch(value_to_add)
                {
                    true
                } else {
                    boxed_sfnum
                        .pair
                        .0
                        .try_add_to_rightmost_value_in_branch(value_to_add)
                }
            }
            Element::RegularNumber(curr_value) => {
                *curr_value += value_to_add;
                true
            }
        }
    }
}

#[derive(PartialEq)]
enum Branch {
    Left,
    Right,
}

struct ExplosionResult {
    is_reduced: bool,
    value_to_add_left: Option<usize>,
    value_to_add_right: Option<usize>,
}

impl ExplosionResult {
    fn new_exploding(left_val: usize, right_val: usize) -> Self {
        ExplosionResult {
            is_reduced: true,
            value_to_add_left: Some(left_val),
            value_to_add_right: Some(right_val),
        }
    }

    fn new_non_reduced() -> Self {
        ExplosionResult {
            is_reduced: false,
            value_to_add_left: None,
            value_to_add_right: None,
        }
    }
}

impl SnailFishNumber {
    fn reduce(&mut self) {
        loop {
            if self.try_reduce_by_exploding(1).is_reduced {
                continue;
            }
            if self.try_reduce_by_splitting() {
                continue;
            }
            break;
        }
    }

    fn try_reduce_by_exploding(&mut self, depth: usize) -> ExplosionResult {
        fn try_reduce_element_by_exploding(element: &mut Element, depth: usize) -> ExplosionResult {
            if let Element::NestedSnailFishNumber(boxed_sfnum) = element {
                if depth == 4 {
                    let number_pair = boxed_sfnum.to_number_pair();
                    let result = ExplosionResult::new_exploding(number_pair.0, number_pair.1);
                    *element = Element::RegularNumber(0);
                    result
                } else {
                    boxed_sfnum.try_reduce_by_exploding(depth + 1)
                }
            } else {
                ExplosionResult::new_non_reduced()
            }
        }

        let left: &mut Element = &mut self.pair.0;
        let right: &mut Element = &mut self.pair.1;

        let mut left_result = try_reduce_element_by_exploding(left, depth);
        if left_result.is_reduced {
            if let Some(value_to_add_right) = left_result.value_to_add_right {
                if right.try_add_to_leftmost_value_in_branch(value_to_add_right) {
                    left_result.value_to_add_right = None;
                }
            }
            return left_result;
        }

        let mut right_result = try_reduce_element_by_exploding(right, depth);
        if right_result.is_reduced {
            if let Some(value_to_add_left) = right_result.value_to_add_left {
                if left.try_add_to_rightmost_value_in_branch(value_to_add_left) {
                    right_result.value_to_add_left = None;
                }
            }
        }
        right_result
    }

    fn try_reduce_by_splitting(&mut self) -> bool {
        fn try_reduce_element_by_splitting(element: &mut Element) -> bool {
            match element {
                Element::RegularNumber(n) if *n >= 10 => {
                    *element = Element::new_number_pair(*n / 2, (*n + 1) / 2);
                    true
                }
                Element::NestedSnailFishNumber(boxed_sfnum) => {
                    boxed_sfnum.try_reduce_by_splitting()
                }
                _ => false,
            }
        }

        try_reduce_element_by_splitting(&mut self.pair.0)
            || try_reduce_element_by_splitting(&mut self.pair.1)
    }

    fn parse(text: &str) -> Self {
        fn parse_element(partial_text: &str, branch: Branch) -> (Element, &str) {
            if partial_text.starts_with('[') {
                // Parse a nested SnailfishNumber
                let (sf_number, rem_text) = parse_nested_sfnumber(partial_text);
                let elem = Element::NestedSnailFishNumber(Box::new(sf_number));
                (elem, rem_text)
            } else {
                // Parse a number
                let delimiter = if branch == Branch::Left { ',' } else { ']' };
                let (num_text, _) = partial_text.split_once(delimiter).unwrap();
                let number: usize = num_text.parse().unwrap();
                (
                    Element::RegularNumber(number),
                    &partial_text[num_text.len()..],
                )
            }
        }

        fn parse_nested_sfnumber(subtext: &str) -> (SnailFishNumber, &str) {
            if !subtext.starts_with('[') {
                panic!("Unexpected element at start of text: {}", subtext);
            }

            let (left_element, subtext) = parse_element(&subtext[1..], Branch::Left);
            if !subtext.starts_with(',') {
                panic!("Unexpected separator before second element: {}", subtext);
            }

            let (right_element, subtext) = parse_element(&subtext[1..], Branch::Right);
            if !subtext.starts_with(']') {
                panic!("Unexpected terminator: {}", subtext);
            }

            let sf_number = SnailFishNumber {
                pair: (left_element, right_element),
            };

            (sf_number, &subtext[1..])
        }

        let (sf_number, remaining_text) = parse_nested_sfnumber(text);
        let non_ws_rem_text = remaining_text.trim();
        if !non_ws_rem_text.is_empty() {
            panic!("Unexpected text that wasn't parsed: {}", non_ws_rem_text);
        }
        sf_number
    }
}

fn sum_of_snailfish_numbers(snailfish_numbers: &[SnailFishNumber]) -> SnailFishNumber {
    let mut sf_num_iter = snailfish_numbers.iter();
    let mut sum_of_sf_nums: SnailFishNumber = sf_num_iter.next().unwrap().clone();
    for sf_num in sf_num_iter {
        sum_of_sf_nums = sum_of_sf_nums + sf_num.clone();
    }
    sum_of_sf_nums
}

fn main() {
    let start_time = Instant::now();

    let contents = fs::read_to_string("data/day18_input.txt").unwrap();
    let snailfish_numbers: Vec<SnailFishNumber> = contents
        .lines()
        .map(SnailFishNumber::parse)
        .collect();
    let sum_of_sf_nums: SnailFishNumber = sum_of_snailfish_numbers(&snailfish_numbers[0..]);
    let magnitude_of_sum = sum_of_sf_nums.magnitude();

    println!("Part 1 answer: {}", magnitude_of_sum);

    let count = snailfish_numbers.len();
    let max_magnitude_of_sum_of_pairs: usize = (0..count)
        .into_iter()
        .flat_map(|i| (0..count).into_iter().map(move |j| (i, j)))
        .filter(|(i, j)| *i != *j)
        .map(|(i, j)| {
            let sum_of_pair: SnailFishNumber =
                snailfish_numbers[i].clone() + snailfish_numbers[j].clone();
            sum_of_pair.magnitude()
        })
        .max()
        .unwrap();

    println!("Part 2 answer: {}", max_magnitude_of_sum_of_pairs);

    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::SnailFishNumber;

    #[test]
    fn test_explode_with_no_left_num_to_add_to() {
        let input_text = "[[[[[9,8],1],2],3],4]";
        let expected_text = "[[[[0,9],2],3],4]";

        let mut input = SnailFishNumber::parse(input_text);
        let result = input.try_reduce_by_exploding(1);
        assert_eq!(result.is_reduced, true);
        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_explode_with_no_right_num_to_add_to() {
        let input_text = "[7,[6,[5,[4,[3,2]]]]]";
        let expected_text = "[7,[6,[5,[7,0]]]]";

        let mut input = SnailFishNumber::parse(input_text);
        let result = input.try_reduce_by_exploding(1);
        assert_eq!(result.is_reduced, true);
        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_explode_with_left_and_right_nums_to_add_to() {
        let input_text = "[[6,[5,[4,[3,2]]]],1]";
        let expected_text = "[[6,[5,[7,0]]],3]";

        let mut input = SnailFishNumber::parse(input_text);
        let result = input.try_reduce_by_exploding(1);
        assert_eq!(result.is_reduced, true);
        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_leftmost_explodes_when_multiple_explosion_candidates() {
        let input_text = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let expected_text = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";

        let mut input = SnailFishNumber::parse(input_text);
        let result = input.try_reduce_by_exploding(1);
        assert_eq!(result.is_reduced, true);
        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_another_explosion() {
        let input_text = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let expected_text = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

        let mut input = SnailFishNumber::parse(input_text);
        let result = input.try_reduce_by_exploding(1);
        assert_eq!(result.is_reduced, true);
        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_split() {
        let input_text = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let expected_text = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";

        let mut input = SnailFishNumber::parse(input_text);
        let result = input.try_reduce_by_splitting();
        assert!(result);
        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_add_and_reduce() {
        let term1_text = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let term2_text = "[1,1]";
        let expected_text = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

        let term1 = SnailFishNumber::parse(term1_text);
        let term2 = SnailFishNumber::parse(term2_text);
        let sum = term1 + term2;

        let expected = SnailFishNumber::parse(expected_text);
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_magnitude_1() {
        let input_text = "[[1,2],[[3,4],5]]";
        let expected_magnitude: usize = 143;

        let input = SnailFishNumber::parse(input_text);
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn test_magnitude_2() {
        let input_text = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let expected_magnitude: usize = 1384;

        let input = SnailFishNumber::parse(input_text);
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn test_magnitude_3() {
        let input_text = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        let expected_magnitude: usize = 445;

        let input = SnailFishNumber::parse(input_text);
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn test_magnitude_4() {
        let input_text = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        let expected_magnitude: usize = 791;

        let input = SnailFishNumber::parse(input_text);
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn test_magnitude_5() {
        let input_text = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let expected_magnitude: usize = 1137;

        let input = SnailFishNumber::parse(input_text);
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn test_magnitude_6() {
        let input_text = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let expected_magnitude: usize = 3488;

        let input = SnailFishNumber::parse(input_text);
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected_magnitude);
    }
}
