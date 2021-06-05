mod ctx_arg;
mod numeric;

pub use ctx_arg::*;
pub use numeric::*;

use std::fmt;
use std::marker::PhantomData;

pub trait Expr<Ctx>: Sized {
    type Output;

    fn eval(self, ctx: &Ctx) -> Self::Output;

    fn equ<E: Expr<Ctx, Output=Self::Output>>(self, other: E) -> Equals<Self::Output, Ctx, Self, E>
        where Self::Output: PartialEq,
    {
        Equals {
            left: self,
            right: other,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_expr_copy {
    ($($typ:ty),* $(,)?) => {
        $(
            impl<Ctx> Expr<Ctx> for $typ {
                type Output = Self;

                fn eval(self, _ctx: &Ctx) -> Self::Output {
                    self
                }
            }
        )*
    };
}

impl_expr_copy!(bool, &str, (), u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MethodCall<Ctx, Receiver, Args, const METHOD: usize> {
    pub receiver: Receiver,
    pub args: Args,
    _marker: PhantomData<Ctx>,
}

pub trait BoolExpr<Ctx>: Expr<Ctx, Output=bool> {
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

impl<Ctx, E: Expr<Ctx, Output=bool>> BoolExpr<Ctx> for E {}

// /// Implements the common traits like Clone, Copy, PartialEq, Eq while excluding type parameters
// /// that don't need to implement those traits
// macro_rules! impl_expr_type_traits {
//     (
//         #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//         #[type_params($($type_param_name:ident),* $(,)?)]
//         #[derive_type_params($($derive_param:ident),* $(,)?)]
//         $tyvis:vis struct $tyname:ident<{$($typaram:tt)*}> {
//             $(
//                 $fieldvis:vis $field:ident : $ty:ty
//             ),*
//             $(,)?
//         }
//     ) => {
//         $tyvis struct $tyname<$($typaram)*> {
//             $($fieldvis $field : $ty),*
//         }

//         impl<$($typaram)*> std::fmt::Debug for $tyname<$($type_param_name),*>

//         {

//         }
//     };
// }

#[derive(Clone, Copy, PartialEq, Eq)]
// #[type_params(T, Ctx, L, R)]
// #[derive_type_params(L, R)]
pub struct Equals<T: PartialEq, Ctx, L: Expr<Ctx, Output=T>, R: Expr<Ctx, Output=T>> {
    pub left: L,
    pub right: R,
    _marker: PhantomData<(T, Ctx)>,
}

impl<T: PartialEq, Ctx, L: Expr<Ctx, Output=T> + fmt::Debug, R: Expr<Ctx, Output=T> + fmt::Debug> fmt::Debug for Equals<T, Ctx, L, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Equals")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<T: PartialEq, Ctx, L: Expr<Ctx, Output=T>, R: Expr<Ctx, Output=T>> Expr<Ctx> for Equals<T, Ctx, L, R> {
    type Output = bool;

    fn eval(self, ctx: &Ctx) -> Self::Output {
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
    type Output = bool;

    fn eval(self, ctx: &Ctx) -> Self::Output {
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
    type Output = bool;

    fn eval(self, ctx: &Ctx) -> Self::Output {
        self.left.eval(ctx) || self.right.eval(ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Not<Ctx, T: BoolExpr<Ctx>> {
    pub value: T,
    _marker: PhantomData<Ctx>,
}

impl<Ctx, T: BoolExpr<Ctx>> Expr<Ctx> for Not<Ctx, T> {
    type Output = bool;

    fn eval(self, ctx: &Ctx) -> Self::Output {
        !self.value.eval(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Field, FieldAccess};

    #[derive(Debug)]
    struct Test {
        age: usize,
        category: &'static str,
    }

    impl FieldAccess<0> for Test {
        type FieldType = usize;

        fn get(&self) -> &Self::FieldType {
            &self.age
        }

        fn get_mut(&mut self) -> &mut Self::FieldType {
            &mut self.age
        }
    }

    impl FieldAccess<1> for Test {
        type FieldType = &'static str;

        fn get(&self) -> &Self::FieldType {
            &self.category
        }

        fn get_mut(&mut self) -> &mut Self::FieldType {
            &mut self.category
        }
    }

    #[derive(Debug, Default)]
    struct TestFields<const ARG_INDEX: usize> {
        age: Field<Test, 0, ARG_INDEX>,
        category: Field<Test, 1, ARG_INDEX>,
    }

    #[test]
    fn test_field_access() {
        let value1 = Test {age: 25, category: "animals"};
        let value2 = Test {age: 25, category: "furniture"};

        // The index in the fields corresponds to the position in the tuple passed to eval
        let fields1 = TestFields::<0>::default();
        let fields2 = TestFields::<1>::default();

        let expr1 = fields1.age.equ(fields2.age);
        let expr2 = fields1.category.equ(fields2.category);
        let expr3 = fields1.age.equ(25usize);

        let ctx = (&value1, &value2);
        assert!(expr1.eval(&ctx));
        assert!(!expr2.eval(&ctx));
        assert!(expr3.eval(&ctx));
    }
}
