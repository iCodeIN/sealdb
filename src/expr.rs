use std::marker::PhantomData;

pub trait Expr<Ctx>: Sized {
    type Type;

    fn eval(self, ctx: &Ctx) -> Self::Type;
}

pub trait BoolExpr<Ctx>: Expr<Ctx, Type=bool> {
    fn eq<E: BoolExpr<Ctx>>(self, other: E) -> Equals<bool, Ctx, Self, E> {
        Equals {
            left: self,
            right: other,
            _marker: PhantomData,
        }
    }

    fn neq<E: BoolExpr<Ctx>>(self, other: E) -> Not<Ctx, Equals<bool, Ctx, Self, E>> {
        self.eq(other).not()
    }

    fn and<E: BoolExpr<Ctx>>(self, other: E) -> And<Ctx, Self, E> {
        And {
            left: self,
            right: other,
            _marker: PhantomData,
        }
    }

    fn or<E: BoolExpr<Ctx>>(self, other: E) -> Or<Ctx, Self, E> {
        Or {
            left: self,
            right: other,
            _marker: PhantomData,
        }
    }

    fn not(self) -> Not<Ctx, Self> {
        Not {
            value: self,
            _marker: PhantomData,
        }
    }
}

impl<Ctx, E: Expr<Ctx, Type=bool>> BoolExpr<Ctx> for E {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equals<T: PartialEq, Ctx, L: Expr<Ctx, Type=T>, R: Expr<Ctx, Type=T>> {
    pub left: L,
    pub right: R,
    _marker: PhantomData<(T, Ctx)>,
}

impl<T: PartialEq, Ctx, L: Expr<Ctx, Type=T>, R: Expr<Ctx, Type=T>> Expr<Ctx> for Equals<T, Ctx, L, R> {
    type Type = bool;

    fn eval(self, ctx: &Ctx) -> Self::Type {
        self.left.eval(ctx) == self.right.eval(ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct And<Ctx, L: BoolExpr<Ctx>, R: BoolExpr<Ctx>> {
    pub left: L,
    pub right: R,
    _marker: PhantomData<Ctx>,
}

impl<Ctx, L: BoolExpr<Ctx>, R: BoolExpr<Ctx>> Expr<Ctx> for And<Ctx, L, R> {
    type Type = bool;

    fn eval(self, ctx: &Ctx) -> Self::Type {
        self.left.eval(ctx) && self.right.eval(ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Or<Ctx, L: BoolExpr<Ctx>, R: BoolExpr<Ctx>> {
    pub left: L,
    pub right: R,
    _marker: PhantomData<Ctx>,
}

impl<Ctx, L: BoolExpr<Ctx>, R: BoolExpr<Ctx>> Expr<Ctx> for Or<Ctx, L, R> {
    type Type = bool;

    fn eval(self, ctx: &Ctx) -> Self::Type {
        self.left.eval(ctx) || self.right.eval(ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Not<Ctx, T: BoolExpr<Ctx>> {
    pub value: T,
    _marker: PhantomData<Ctx>,
}

impl<Ctx, T: BoolExpr<Ctx>> Expr<Ctx> for Not<Ctx, T> {
    type Type = bool;

    fn eval(self, ctx: &Ctx) -> Self::Type {
        !self.value.eval(ctx)
    }
}
