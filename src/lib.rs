use std::{
    cmp::Ordering,
    collections::VecDeque,
    iter::{once, repeat},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        let integers = [
            TwoAdic::OI([Bit::O, Bit::O].into()), // 000111... = -8
            TwoAdic::OI([Bit::O].into()),         // 0011... = -4
            TwoAdic::OI([Bit::I].into()),         // 1011... = -3
            TwoAdic::OI([].into()),               // 0111... = -2
            TwoAdic::II,                          // 1111... = -1
            TwoAdic::OO,                          // 0000... = 0
            TwoAdic::IO([].into()),               // 1000... = 1
            TwoAdic::IO([Bit::O].into()),         // 0100... = 2
            TwoAdic::IO([Bit::I].into()),         // 1100... = 3
            TwoAdic::IO([Bit::I, Bit::I].into()), // 1110... = 7
        ];

        for (i, v) in integers.iter().enumerate() {
            for w in integers.iter().skip(i) {
                assert!(v <= w, "{:?} <= {:?}", v, w);
            }
        }
    }

    #[test]
    fn from_i32() {
        let integers = [
            (-8, TwoAdic::OI([Bit::O, Bit::O].into())), // 000111... = -8
            (-4, TwoAdic::OI([Bit::O].into())),         // 0011... = -4
            (-3, TwoAdic::OI([Bit::I].into())),         // 1011... = -3
            (-2, TwoAdic::OI([].into())),               // 0111... = -2
            (-1, TwoAdic::II),                          // 1111... = -1
            (0, TwoAdic::OO),                           // 0000... = 0
            (1, TwoAdic::IO([].into())),                // 1000... = 1
            (2, TwoAdic::IO([Bit::O].into())),          // 0100... = 2
            (3, TwoAdic::IO([Bit::I].into())),          // 1100... = 3
            (7, TwoAdic::IO([Bit::I, Bit::I].into())),  // 1110... = 7
        ];

        for (f, t) in integers.into_iter() {
            assert_eq!(TwoAdic::from(f), t);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Bit {
    O,
    I,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TwoAdic {
    OI(VecDeque<Bit>), // least significant to most significant, followed by 0 and then infinitely many 1s. Represents negative integers less than -1
    II,                // infinitely many 1s. Represents -1
    OO,                // infinitely many 0s. Represents 0
    IO(VecDeque<Bit>), // least significant to most significant, followed by 1 and then infinitely many 0s. Represents positve integers
}

static EMPTY_BITS: VecDeque<Bit> = VecDeque::new();

impl TwoAdic {
    fn bits(&self) -> impl Iterator<Item = Bit> + '_ {
        match self {
            Self::OI(bits) => bits
                .iter()
                .cloned()
                .chain(once(Bit::O))
                .chain(repeat(Bit::I)),
            Self::II => EMPTY_BITS
                .iter()
                .cloned()
                .chain(once(Bit::I))
                .chain(repeat(Bit::I)),
            Self::OO => EMPTY_BITS
                .iter()
                .cloned()
                .chain(once(Bit::O))
                .chain(repeat(Bit::O)),
            Self::IO(bits) => bits
                .iter()
                .cloned()
                .chain(once(Bit::I))
                .chain(repeat(Bit::O)),
        }
    }
}

impl Ord for TwoAdic {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::OI(lhs) => match other {
                Self::OI(rhs) => {
                    let len = lhs.len().max(rhs.len());
                    let lhs = lhs
                        .iter()
                        .cloned()
                        .chain([Bit::O, Bit::I])
                        .chain(vec![Bit::I; len - lhs.len()])
                        .rev();
                    let rhs = rhs
                        .iter()
                        .cloned()
                        .chain([Bit::O, Bit::I])
                        .chain(vec![Bit::I; len - rhs.len()])
                        .rev();
                    lhs.cmp(rhs)
                }
                Self::II => Ordering::Less,
                Self::OO => Ordering::Less,
                Self::IO(_) => Ordering::Less,
            },
            Self::II => match other {
                Self::OI(_) => Ordering::Greater,
                Self::II => Ordering::Equal,
                Self::OO => Ordering::Less,
                Self::IO(_) => Ordering::Less,
            },
            Self::OO => match other {
                Self::OI(_) => Ordering::Greater,
                Self::II => Ordering::Greater,
                Self::OO => Ordering::Equal,
                Self::IO(_) => Ordering::Less,
            },
            Self::IO(lhs) => match other {
                Self::OI(_) => Ordering::Greater,
                Self::II => Ordering::Greater,
                Self::OO => Ordering::Greater,
                Self::IO(rhs) => {
                    let len = lhs.len().max(rhs.len());
                    let lhs = lhs
                        .iter()
                        .cloned()
                        .chain([Bit::I, Bit::O])
                        .chain(vec![Bit::O; len - lhs.len()])
                        .rev();
                    let rhs = rhs
                        .iter()
                        .cloned()
                        .chain([Bit::I, Bit::O])
                        .chain(vec![Bit::O; len - rhs.len()])
                        .rev();
                    lhs.cmp(rhs)
                }
            },
        }
    }
}

impl PartialOrd for TwoAdic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<i32> for TwoAdic {
    fn from(value: i32) -> Self {
        let bits_reversed = (0..32).rev().map(|i| {
            if ((value >> i) & 1) == 1 {
                Bit::I
            } else {
                Bit::O
            }
        });
        match value {
            (..=-2) => {
                let bits_reversed: VecDeque<_> =
                    bits_reversed.skip_while(|b| *b == Bit::I).skip(1).collect();
                let bits = bits_reversed.into_iter().rev().collect();
                Self::OI(bits)
            }
            -1 => Self::II,
            0 => Self::OO,
            (1..) => {
                let bits_reversed: VecDeque<_> =
                    bits_reversed.skip_while(|b| *b == Bit::O).skip(1).collect();
                let bits = bits_reversed.into_iter().rev().collect();
                Self::IO(bits)
            }
        }
    }
}
