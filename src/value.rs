use crate::board::moves::Move;

#[derive(Debug, Clone, Copy, Default)]
pub enum Value {
    #[default]
    Draw,
    Evaluation(i32),
    Mate(i32),
}

impl Value {
    pub fn mate(plies: usize) -> Self {
        Value::Mate((plies as i32 + 1) / 2)
    }

    pub(crate) fn to_i32(self) -> i32 {
        match self {
            Value::Mate(turns) => turns.signum() * 1_000_000 - turns,
            Value::Evaluation(v) => v,
            Value::Draw => 0,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.to_i32() == other.to_i32()
    }
}

impl Eq for Value {}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Mate(turns) => Value::Mate(-turns),
            Value::Evaluation(v) => Value::Evaluation(-v),
            Value::Draw => self,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_i32().cmp(&other.to_i32())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Draw => write!(f, "cp {}", 0),
            Value::Evaluation(v) => write!(f, "cp {}", v),
            Value::Mate(t) => write!(f, "mate {}", t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MoveWithValue {
    pub mv: Move,
    pub value: Value,
}

impl Ord for MoveWithValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for MoveWithValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        board::{
            moves::{Move, MoveKind},
            square::Square,
        },
        value::Value,
    };

    use super::MoveWithValue;

    #[test]
    fn equal() {
        assert_eq!(Value::Draw, Value::Draw);
        assert_eq!(Value::Mate(2), -Value::Mate(-2));
    }

    #[test]
    fn neg() {
        assert_eq!(Value::Mate(2), -Value::Mate(-2));
    }

    #[test]
    fn comparison() {
        assert!(Value::Draw > Value::Evaluation(-100));
        assert!(Value::Mate(2) > Value::Evaluation(-100));
        assert!(Value::Mate(-2) < Value::Mate(-5));
    }

    #[test]
    fn move_with_value() {
        let a = MoveWithValue {
            mv: Move::new(Square::A1, Square::A2, MoveKind::Quiet),
            value: Value::Evaluation(100),
        };
        let b = MoveWithValue {
            mv: Move::new(Square::A1, Square::A2, MoveKind::Quiet),
            value: Value::Evaluation(200),
        };

        assert!(b > a);
        assert!(b >= a);
        assert!(a < b);
    }

    #[test]
    fn move_with_value2() {
        let a = MoveWithValue {
            mv: Move::new(Square::A1, Square::A2, MoveKind::Quiet),
            value: Value::Evaluation(100),
        };
        let b = MoveWithValue {
            mv: Move::new(Square::A1, Square::A2, MoveKind::Quiet),
            value: Value::Mate(2),
        };

        assert!(b >= a);
        assert!(a < b);
    }
}
