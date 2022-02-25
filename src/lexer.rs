#[derive(PartialEq)]
pub enum Lexeme {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
    Hundred,
    Thousand,
    Million,
    Billion,
    Trillion,
    And,
    Hyphen
}

impl Lexeme {
    fn from_str(s: &str) -> Result<Lexeme, String> {
        Ok(match s {
            "one" => Lexeme::One,
            "two" => Lexeme::Two,
            "three" => Lexeme::Three,
            "four" => Lexeme::Four,
            "five" => Lexeme::Five,
            "six" => Lexeme::Six,
            "seven" => Lexeme::Seven,
            "eight" => Lexeme::Eight,
            "nine" => Lexeme::Nine,
            "ten" => Lexeme::Ten,
            "eleven" => Lexeme::Eleven,
            "twelve" => Lexeme::Twelve,
            "thirteen" => Lexeme::Thirteen,
            "fourteen" => Lexeme::Fourteen,
            "fifteen" => Lexeme::Fifteen,
            "sixteen" => Lexeme::Sixteen,
            "seventeen" => Lexeme::Seventeen,
            "eighteen" => Lexeme::Eighteen,
            "nineteen" => Lexeme::Nineteen,
            "twenty" => Lexeme::Twenty,
            "thirty" => Lexeme::Thirty,
            "forty" => Lexeme::Forty,
            "fifty" => Lexeme::Fifty,
            "sixty" => Lexeme::Sixty,
            "seventy" => Lexeme::Seventy,
            "eighty" => Lexeme::Eight,
            "ninety" => Lexeme::Ninety,
            "hundred" => Lexeme::Hundred,
            "thousand" => Lexeme::Thousand,
            "million" => Lexeme::Million,
            "billion" => Lexeme::Billion,
            "trillion" => Lexeme::Trillion,
            "and" => Lexeme::And,
            "-" => Lexeme::Hyphen,
            _ => return Err(format!("Unrecognized token: {s}"))
        })
    }
}

pub fn lex(line: &str) -> Result<Vec<Lexeme>, String> {
    let mut lexemes = Vec::new();

    // This is our stack, where we store characters before converting them
    let mut stack = String::new();

    // Helper function for pushing lexemes from the stack
    let push_lexeme = |stack: &mut String, ls: &mut Vec<Lexeme>| -> Result<(), String> {
        if !stack.is_empty() {
            ls.push(Lexeme::from_str(stack)?);
            stack.clear();
        }

        Ok(())
    };

    for c in line.chars() {
        // White space is always a separator, push whatever is on the stack
        // And clear it for further use
        if c.is_whitespace() {
            push_lexeme(&mut stack, &mut lexemes)?;
            continue;
        }

        match c {
            '-' => {
                // A Hyphen also acts as a separator, so we push whatever was on
                // the stack and then manually add the hyphen to lexemes
                push_lexeme(&mut stack, &mut lexemes)?;
                lexemes.push(Lexeme::Hyphen);
            }
            _ => {
                stack.push(c);
            }
        }
    }

    // Push whatever characters are left on the stack
    push_lexeme(&mut stack, &mut lexemes)?;

    Ok(lexemes)
}
