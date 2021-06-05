use std::{cmp, marker::PhantomData};

use super::{Expr, MethodCall};

//TODO: Remove these and use string literals when const generics support those
const M_MIN: usize = 0;
const M_MAX: usize = 1;

/// Methods shared by all numeric types
pub trait NumericExpr<Ctx, N>: Expr<Ctx, Output=N> {
    fn min<E: NumericExpr<Ctx, N>>(self, other: E) -> MethodCall<Ctx, Self, (E,), M_MIN>
        where N: Ord,
    {
        MethodCall {
            receiver: self,
            args: (other,),
            _marker: PhantomData,
        }
    }

    fn max<E: NumericExpr<Ctx, N>>(self, other: E) -> MethodCall<Ctx, Self, (E,), M_MAX>
        where N: Ord,
    {
        MethodCall {
            receiver: self,
            args: (other,),
            _marker: PhantomData,
        }
    }
}

impl<Ctx, N, E: Expr<Ctx, Output=N>> NumericExpr<Ctx, N> for E {}

impl<Ctx, N: Ord, Receiver: Expr<Ctx, Output=N>, Arg: Expr<Ctx, Output=N>> Expr<Ctx> for MethodCall<Ctx, Receiver, (Arg,), M_MIN> {
    type Output = N;

    fn eval(self, ctx: &Ctx) -> Self::Output {
        cmp::min(self.receiver.eval(ctx), self.args.0.eval(ctx))
    }
}

impl<Ctx, N: Ord, Receiver: Expr<Ctx, Output=N>, Arg: Expr<Ctx, Output=N>> Expr<Ctx> for MethodCall<Ctx, Receiver, (Arg,), M_MAX> {
    type Output = N;

    fn eval(self, ctx: &Ctx) -> Self::Output {
        cmp::max(self.receiver.eval(ctx), self.args.0.eval(ctx))
    }
}
