pub trait CtxArg<const ARG_INDEX: usize>: Copy {
    type Output;

    fn get(self) -> Self::Output;
}

impl<'a, Ctx, const ARG_INDEX: usize> CtxArg<ARG_INDEX> for &'a Ctx
    where Ctx: CtxArg<ARG_INDEX>,
{
    type Output = <Ctx as CtxArg<ARG_INDEX>>::Output;

    fn get(self) -> Self::Output {
        (*self).get()
    }
}

impl<'a, T> CtxArg<0> for (&'a T,) {
    type Output = &'a T;

    fn get(self) -> Self::Output {
        self.0
    }
}

impl<'a, T, U> CtxArg<0> for (&'a T, &'a U) {
    type Output = &'a T;

    fn get(self) -> Self::Output {
        self.0
    }
}

impl<'a, T, U> CtxArg<1> for (&'a T, &'a U) {
    type Output = &'a U;

    fn get(self) -> Self::Output {
        self.1
    }
}

impl<'a, T, U, V> CtxArg<0> for (&'a T, &'a U, &'a V) {
    type Output = &'a T;

    fn get(self) -> Self::Output {
        self.0
    }
}

impl<'a, T, U, V> CtxArg<1> for (&'a T, &'a U, &'a V) {
    type Output = &'a U;

    fn get(self) -> Self::Output {
        self.1
    }
}

impl<'a, T, U, V> CtxArg<2> for (&'a T, &'a U, &'a V) {
    type Output = &'a V;

    fn get(self) -> Self::Output {
        self.2
    }
}
