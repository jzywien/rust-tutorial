pub fn get_suffix_for_num(n :u32) -> String {
    let last_num = get_nth_digit(n, -1);
    let is_teens = (n > 9) && (get_nth_digit(n, -2) == 1);

    if is_teens {
        String::from("th")
    } else {
        match last_num {
            1 => String::from("st"),
            2 => String::from("nd"),
            3 => String::from("rd"),
            _ => String::from("th")
        }
    }
}

fn get_nth_digit(num: u32, ndx: i32) -> u32 {
    let num_str = num.to_string();
    let len = num_str.len();
    let is_reverse = ndx < 0;
    let ndx = ndx.abs() as usize;
    let digit_str = if !is_reverse {
        &num_str[ndx..ndx+1]
    } else {
        &num_str[len-ndx..len-ndx+1]
    };

    digit_str.parse::<u32>().expect("Nth Digit must be number!")
}

#[cfg(test)]
mod number_tests {
    use number;

    #[test]
    fn test_get_nth_digit() {
        assert_eq!(number::get_nth_digit(12, 1), 2);
        assert_eq!(number::get_nth_digit(1, 0), 1);
        assert_eq!(number::get_nth_digit(12, -1), 2);
        assert_eq!(number::get_nth_digit(12, -2), 1);
        assert_eq!(number::get_nth_digit(12345, 2), 3);
        assert_eq!(number::get_nth_digit(12345, -2), 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_index_ltor() {
        number::get_nth_digit(3, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_index_rtol() {
        number::get_nth_digit(3, -2);
    }
}
