use std::collections::HashMap;

fn increasing(num: &String) -> bool {
    num.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .all(|it| it[0] <= it[1])
}

fn two_adjacent_eq(num: &String) -> bool {
    num.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|it| it[0] == it[1])
}

fn one_double_pair(num: &String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();

    num.chars().for_each(|it| {
        if map.contains_key(&it) {
            *map.get_mut(&it).unwrap() += 1;
        } else {
            map.insert(it, 1);
        }
    });

    map.values().any(|it| *it == 2)
}

fn is_valid(num: String) -> bool {
    if !increasing(&num) {
        return false;
    }

    if !two_adjacent_eq(&num) {
        return false;
    }

    true
}

fn is_valid_enh(num: String) -> bool {
    if !increasing(&num) {
        return false;
    }

    if !one_double_pair(&num) {
        return false;
    }

    true
}

fn main() {
    let inp_min = 156218;
    let inp_max = 652527;

    {
        let mut result = 0;
        for i in inp_min..=inp_max {
            if is_valid(i.to_string()) {
                result += 1;
            }
        }

        println!("Part One: {}", result);
    }

    {
        let mut result = 0;
        for i in inp_min..=inp_max {
            if is_valid_enh(i.to_string()) {
                result += 1;
            }
        }

        println!("Part Two: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(is_valid(111111.to_string()));
        assert!(!is_valid(223450.to_string()));
        assert!(!is_valid(123789.to_string()));
    }

    #[test]
    fn test_two() {
        assert!(is_valid_enh(112233.to_string()));
        assert!(!is_valid_enh(123444.to_string()));
        assert!(is_valid_enh(111122.to_string()));
    }
}
