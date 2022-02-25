use crate::lexer::Lexeme;

pub trait Parse {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> where Self: Sized;
}

pub trait ToNum {
    fn to_num(&self) -> usize;
}

#[derive(Debug)]
pub enum Ones {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Parse for Ones {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        Some((match l.get(0) {
            Some(Lexeme::One) => Self::One,
            Some(Lexeme::Two) => Self::Two,
            Some(Lexeme::Three) => Self::Three,
            Some(Lexeme::Four) => Self::Four,
            Some(Lexeme::Five) => Self::Five,
            Some(Lexeme::Six) => Self::Six,
            Some(Lexeme::Seven) => Self::Seven,
            Some(Lexeme::Eight) => Self::Eight,
            Some(Lexeme::Nine) => Self::Nine,
            _ => return None
        }, 1))
    }
}

impl ToNum for Ones {
    fn to_num(&self) -> usize {
        match self {
            Ones::One => 1,
            Ones::Two => 2,
            Ones::Three => 3,
            Ones::Four => 4,
            Ones::Five => 5,
            Ones::Six => 6,
            Ones::Seven => 7,
            Ones::Eight => 8,
            Ones::Nine => 9,
        }
    }
}

#[derive(Debug)]
pub enum Teens {
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen
}

impl Parse for Teens {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        Some((match l.get(0) {
            Some(Lexeme::Ten) => Self::Ten,
            Some(Lexeme::Eleven) => Self::Eleven,
            Some(Lexeme::Twelve) => Self::Twelve,
            Some(Lexeme::Thirteen) => Self::Thirteen,
            Some(Lexeme::Fourteen) => Self::Fourteen,
            Some(Lexeme::Fifteen) => Self::Fifteen,
            Some(Lexeme::Sixteen) => Self::Sixteen,
            Some(Lexeme::Seventeen) => Self::Seventeen,
            Some(Lexeme::Eighteen) => Self::Eighteen,
            Some(Lexeme::Nineteen) => Self::Nineteen,
            _ => return None
        }, 1))
    }
}

impl ToNum for Teens {
    fn to_num(&self) -> usize {
        match self {
            Teens::Ten => 10,
            Teens::Eleven => 11,
            Teens::Twelve => 12,
            Teens::Thirteen => 13,
            Teens::Fourteen => 14,
            Teens::Fifteen => 15,
            Teens::Sixteen => 16,
            Teens::Seventeen => 17,
            Teens::Eighteen => 18,
            Teens::Nineteen => 19
        }
    }
}

#[derive(Debug)]
pub enum TensUnit {
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety
}

impl Parse for TensUnit {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        Some((match l.get(0) {
            Some(Lexeme::Twenty) => Self::Twenty,
            Some(Lexeme::Thirty) => Self::Thirty,
            Some(Lexeme::Forty) => Self::Forty,
            Some(Lexeme::Fifty) => Self::Fifty,
            Some(Lexeme::Sixty) => Self::Sixty,
            Some(Lexeme::Seventy) => Self::Seventy,
            Some(Lexeme::Eighty) => Self::Eighty,
            Some(Lexeme::Ninety) => Self::Ninety,
            _ => return None
        }, 1))
    }
}

impl ToNum for TensUnit {
    fn to_num(&self) -> usize {
        match self {
            Self::Twenty => 20,
            Self::Thirty => 30,
            Self::Forty => 40,
            Self::Fifty => 50,
            Self::Sixty => 60,
            Self::Seventy => 70,
            Self::Eighty => 80,
            Self::Ninety => 90
        }
    }
}

#[derive(Debug)]
pub enum TripleUnit {
    Thousand,
    Million,
    Billion,
    Trillion,
}

impl Parse for TripleUnit {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        Some((match l.get(0) {
            Some(Lexeme::Thousand) => Self::Thousand,
            Some(Lexeme::Million) => Self::Million,
            Some(Lexeme::Billion) => Self::Billion,
            Some(Lexeme::Trillion) => Self::Trillion,
            _ => return None
        }, 1))
    }
}

impl ToNum for TripleUnit {
    fn to_num(&self) -> usize {
        match self {
            Self::Thousand => 1000,
            Self::Million => 1000000,
            Self::Billion => 1000000000,
            Self::Trillion => 1000000000000,
        }
    }
}

#[derive(Debug)]
pub enum Tens {
    UnitOnes(TensUnit, Ones),
    Unit(TensUnit),
    Teens(Teens),
    Ones(Ones)
}

impl Parse for Tens {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some((tens_unit, n)) = TensUnit::parse(&l[tokens..]) {
            tokens += n;

            if let Some(Lexeme::Hyphen) = l.get(tokens) {
                tokens += 1;

                if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
                    tokens += n;
                    return Some((Self::UnitOnes(tens_unit, ones), tokens));
                }

                tokens -= 1;
            }

            if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
                tokens += n;
                return Some((Self::UnitOnes(tens_unit, ones), tokens));
            }

            return Some((Self::Unit(tens_unit), tokens));
        }

        if let Some((teens, n)) = Teens::parse(&l[tokens..]) {
            tokens += n;
            return Some((Self::Teens(teens), tokens));
        }

        if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
            tokens += n;
            return Some((Self::Ones(ones), tokens));
        }

        None
    }
}

impl ToNum for Tens {
    fn to_num(&self) -> usize {
        match self {
            Self::Ones(ones) => ones.to_num(),
            Self::Teens(teens) => teens.to_num(),
            Self::Unit(unit) => unit.to_num(),
            Self::UnitOnes(unit, ones) => unit.to_num() + ones.to_num()
        }
    }
}

#[derive(Debug)]
pub enum Hundreds {
    HundredsTens(Ones, Tens),
    Hundreds(Ones),
    Tens(Tens)
}

impl Parse for Hundreds {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
            tokens += n;
            if Some(&Lexeme::Hundred) == l.get(tokens) {
                tokens += 1;

                if Some(&Lexeme::And) == l.get(tokens) {
                    tokens += 1;

                    if let Some((tens, n)) = Tens::parse(&l[tokens..]) {
                        tokens += n;
                        return Some((Hundreds::HundredsTens(ones, tens), tokens))
                    }
                    tokens -= 1;
                }


                if let Some((tens, n)) = Tens::parse(&l[tokens..]) {
                    tokens += n;
                    return Some((Self::HundredsTens(ones, tens), tokens));
                }

                return Some((Self::Hundreds(ones), tokens));
            }
            tokens -= n;
        }

        if let Some((tens, n)) = Tens::parse(&l[tokens..]) {
            tokens += n;
            return Some((Self::Tens(tens), tokens));
        }

        None
    }
}

impl ToNum for Hundreds {
    fn to_num(&self) -> usize {
        match self {
            Self::HundredsTens(ones, tens) => ones.to_num()*100 + tens.to_num(),
            Self::Hundreds(ones) => ones.to_num()*100,
            Self::Tens(tens) => tens.to_num()
        }
    }
}

#[derive(Debug)]
pub enum Num {
    TripleNum(Hundreds, TripleUnit, Box<Num>),
    Triple(Hundreds, TripleUnit),
    Hundreds(Hundreds)
}

impl Parse for Num {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some((hundreds, n)) = Hundreds::parse(&l[tokens..]) {
            tokens += n;
            if let Some((triple_unit, n)) = TripleUnit::parse(&l[tokens..]) {
                tokens += n;

                if Some(&Lexeme::And) == l.get(tokens) {
                    tokens += 1;

                    if let Some((num, n)) = Num::parse(&l[tokens..]) {
                        tokens += n;
                        return Some((Self::TripleNum(hundreds, triple_unit, Box::new(num)), tokens));
                    }

                    tokens -= 1;
                }

                if let Some((num, n)) = Num::parse(&l[tokens..]) {
                    tokens += n;
                    return Some((Self::TripleNum(hundreds, triple_unit, Box::new(num)), tokens));
                }

                return Some((Self::Triple(hundreds, triple_unit), tokens));
            }
            return Some((Self::Hundreds(hundreds), tokens));
        }

        None
    }
}

impl ToNum for Num {
    fn to_num(&self) -> usize {
        match self {
            Self::TripleNum(h, unit, num) => h.to_num()*unit.to_num()+num.to_num(),
            Self::Triple(h, unit) => h.to_num()*unit.to_num(),
            Self::Hundreds(h) => h.to_num()
        }
    }
}
