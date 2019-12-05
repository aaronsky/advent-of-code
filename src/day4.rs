
    // 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
    // 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
    // 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).

use digits_iterator::*;
use std::u8::MAX as U8_MAX;

fn is_valid_password(candidate: u32, range_start: u32, range_end: u32) -> bool {
    if candidate.digits().count() != 6 {
        return false;
    } else if candidate < range_start || candidate > range_end {
        return false;
    }
    let mut successful_candidate = true;
    let mut adjacent_sequence = String::new();
    let mut found_a_sequence_of_exactly_two_digits = false;
    let mut last_number = U8_MAX;
    for digit in candidate.digits() {
        if last_number != U8_MAX && digit < last_number {
            successful_candidate = false;
            break;
        }

        if last_number == U8_MAX {
            adjacent_sequence.push(digit as char);
        } else {
            if digit < last_number {
                successful_candidate = false;
                break;
            } else if digit == last_number {
                adjacent_sequence.push(digit as char);
            } else {
                if adjacent_sequence.len() == 2 {
                    found_a_sequence_of_exactly_two_digits = true;
                }
                adjacent_sequence = String::new();
                adjacent_sequence.push(digit as char);
            }
        }
        // if last_number == digit {
        //     if adjacent_sequence.is_empty() {
        //         adjacent_sequence.push(last_number as char);
        //         adjacent_sequence.push(digit as char);
        //     } else if adjacent_sequence.chars().last().unwrap() as u8 != digit {
        //         adjacent_sequences.push(adjacent_sequence);
        //         adjacent_sequence = String::new();
        //     } else {
        //         adjacent_sequence.push(digit as char);
        //     }
        // } else {
        //     if !adjacent_sequence.is_empty() {
        //         adjacent_sequences.push(adjacent_sequence);
        //         adjacent_sequence = String::new();
        //     }
        // }
        last_number = digit;
    }
    
    found_a_sequence_of_exactly_two_digits |= adjacent_sequence.len() == 2;

    if successful_candidate && found_a_sequence_of_exactly_two_digits {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> (u32, u32) {
        let range: Vec<u32> = input.split("-").take(2).map(str_to_u32).collect();
        (range[0], range[1])
    }

    fn str_to_u32(string: &str) -> u32 {
        string.parse::<u32>().unwrap()
    }

    #[test]
    fn test_advent_puzzle() {
        let (start, end) = parse("152085-670283");
        let possibilities_count = (start..end)
            .filter(|num| is_valid_password(*num, start, end))
            .count();
        println!("{}", possibilities_count);
    }

    #[test]
    fn smoke_simple_program_1() {
        assert!(is_valid_password(112233, 112233, 112233));
    }

    #[test]
    fn smoke_simple_program_2() {
        assert!(!is_valid_password(123444, 123444, 123444));
    }

    #[test]
    fn smoke_simple_program_3() {
        assert!(is_valid_password(111122, 111122, 111122));
    }
}