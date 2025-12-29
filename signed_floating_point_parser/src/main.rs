#[allow(dead_code)]
#[derive(Debug)]
enum DecimalNumber {
    Whole { sign: Sign, value: String },
    Decimal { sign: Sign, whole: String, fraction: String },
}

#[derive(Debug)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Expression {
    Single(DecimalNumber),
    Operation {
        left: DecimalNumber,
        op: Operator,
        right: DecimalNumber,
    },
}

#[allow(dead_code)]
trait SplitAtChecked {
    fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)>;
}

impl SplitAtChecked for &str {
    fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)> {
        if mid <= self.len() {
            Some(self.split_at(mid))
        } else {
            None
        }
    }
}

fn parse_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_number(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    if input.len() == 1 {
        return parse_digit(input.chars().next().unwrap());
    }

    if let Some((prefix, last)) = input.split_at_checked(input.len() - 1) {
        return parse_number(prefix) && parse_digit(last.chars().next().unwrap());
    }

    false
}

fn parse_decimal(input: &str) -> Option<DecimalNumber> {
    if input.is_empty() {
        return None;
    }

    // Check for sign
    let (sign, number_part) = if input.starts_with('-') {
        (Sign::Negative, &input[1..])
    } else if input.starts_with('+') {
        (Sign::Positive, &input[1..])
    } else {
        (Sign::Positive, input)
    };

    if number_part.is_empty() {
        return None;
    }

    // Check for decimal point
    if let Some((whole_part, fraction_part)) = number_part.split_once('.') {
        if parse_number(whole_part) && parse_number(fraction_part) {
            Some(DecimalNumber::Decimal {
                sign,
                whole: whole_part.to_string(),
                fraction: fraction_part.to_string(),
            })
        } else {
            None
        }
    } else if parse_number(number_part) {
        Some(DecimalNumber::Whole {
            sign,
            value: number_part.to_string(),
        })
    } else {
        None
    }
}

fn parse_expression(input: &str) -> Option<Expression> {
    // Try to find operators, but be careful with leading signs
    for (i, c) in input.char_indices() {
        if i == 0 {
            continue; // Skip first character to avoid confusing sign with operator
        }

        // Check if this is an operator (not a sign)
        if matches!(c, '+' | '-' | '*' | '/') {
            // Make sure the previous character isn't an operator (to avoid treating sign as operator)
            let prev_char = input[..i].chars().last();
            if let Some(pc) = prev_char {
                if !matches!(pc, '+' | '-' | '*' | '/') {
                    let left = input[..i].trim();
                    let right = input[i + 1..].trim();

                    let op = match c {
                        '+' => Operator::Add,
                        '-' => Operator::Sub,
                        '/' => Operator::Div,
                        '*' => Operator::Mul,
                        _ => unreachable!(),
                    };

                    if let (Some(lnum), Some(rnum)) = (parse_decimal(left), parse_decimal(right)) {
                        return Some(Expression::Operation {
                            left: lnum,
                            op,
                            right: rnum,
                        });
                    }
                }
            }
        }
    }

    parse_decimal(input).map(Expression::Single)
}

fn main() {
    let tests = [
        "123",
        "-456",
        "+789",
        "123.45",
        "-123.45",
        "+123.45",
        "10.5 + 20.3",
        "-15.2 * 3.7",
        "100 - -50",
        "-25.5 / 5",
        "12.3.4",
        "-",
        "+",
        "abc",
    ];

    for &t in &tests {
        match parse_expression(t) {
            Some(expr) => println!("{t}: valid -> {:#?}", expr),
            None => println!("{t}: invalid"),
        }
    }
}
