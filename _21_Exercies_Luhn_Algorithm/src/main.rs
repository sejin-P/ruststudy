pub fn luhn(cc_number: &str) -> bool {
    let length = cc_number.len();
    let mut totalBlankCnt = 0;
    let mut sumLuhn = 0;
    for (i, ch) in cc_number.chars().rev().enumerate() {
        if ch == ' ' {
            totalBlankCnt += 1;
            continue
        }
        let digit = ch.to_digit(10);
        match digit {
            Some(d) => {
                sumLuhn += if (i-totalBlankCnt) %2 == 1 {
                    let doubled = d*2;
                    doubled/10 + doubled%10
                } else {
                    d
                }
            }
            None => return false
        }
    }

    if length - totalBlankCnt < 2 {
        return false
    }

    if sumLuhn%10 == 0 {
        return true
    }

    return false
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

fn main() {
}