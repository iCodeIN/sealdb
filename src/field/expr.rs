pub trait Expr: Sized {
    type Type;
}

pub trait BoolExpr: Sized {
    fn eq<E: BoolExpr>(self, other: E) -> Equals<Self, E> {
        Equals {
            left: self,
            right: other,
        }
    }

    fn neq<E: BoolExpr>(self, other: E) -> Not<Equals<Self, E>> {
        self.eq(other).not()
    }

    fn and<E: BoolExpr>(self, other: E) -> And<Self, E> {
        And {
            left: self,
            right: other,
        }
    }

    fn or<E: BoolExpr>(self, other: E) -> Or<Self, E> {
        Or {
            left: self,
            right: other,
        }
    }

    fn not(self) -> Not<Self> {
        Not {value: self}
    }
}

impl<T: Expr<Type=bool>> BoolExpr for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equals<L: BoolExpr, R: BoolExpr> {
    left: L,
    right: R,
}

impl<L: BoolExpr, R: BoolExpr> Expr for Equals<L, R> {
    type Type = bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct And<L: BoolExpr, R: BoolExpr> {
    left: L,
    right: R,
}

impl<L: BoolExpr, R: BoolExpr> Expr for And<L, R> {
    type Type = bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Or<L: BoolExpr, R: BoolExpr> {
    left: L,
    right: R,
}

impl<L: BoolExpr, R: BoolExpr> Expr for Or<L, R> {
    type Type = bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Not<T: BoolExpr> {
    value: T,
}

impl<T: BoolExpr> Expr for Not<T> {
    type Type = bool;
}
