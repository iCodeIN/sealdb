use std::fmt;
use std::marker::PhantomData;

use crate::Expr;

pub trait FieldAccess<const FIELD_NAME: usize> {
    type FieldType;

    /// Returns a reference to this field in `self`
    fn get(&self) -> &Self::FieldType;
    /// Returns a mutable reference to this field in `self`
    fn get_mut(&mut self) -> &mut Self::FieldType;

    fn set(&mut self, field_value: Self::FieldType) {
        *self.get_mut() = field_value;
    }
}

//TODO: `FIELD_NAME` should be `&str` once that is supported by const generics
pub struct Field<T, const FIELD_NAME: usize>
    where T: FieldAccess<FIELD_NAME>,
{
    _marker: PhantomData<T>,
}

impl<T, const FIELD_NAME: usize> Default for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME>,
{
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T, const FIELD_NAME: usize> fmt::Debug for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field({}.{})", std::any::type_name::<T>(), FIELD_NAME)
    }
}

impl<T, const FIELD_NAME: usize> Clone for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const FIELD_NAME: usize> Copy for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME> {}

impl<T, const FIELD_NAME: usize> PartialEq for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME>,
{
    fn eq(&self, _other: &Self) -> bool {
        // Same type `T` and same `FIELD_NAME` is always the same
        true
    }
}

impl<T, const FIELD_NAME: usize> Eq for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME> {}

impl<T, const FIELD_NAME: usize> Expr<T> for Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME>,
          <T as FieldAccess<FIELD_NAME>>::FieldType: Copy
{
    type Output = <T as FieldAccess<FIELD_NAME>>::FieldType;

    fn eval(self, ctx: &T) -> Self::Output {
        *self.get(ctx)
    }
}

impl<T, const FIELD_NAME: usize> Field<T, FIELD_NAME>
    where T: FieldAccess<FIELD_NAME>,
{
    pub fn get(self, value: &T) -> &<T as FieldAccess<FIELD_NAME>>::FieldType {
        value.get()
    }

    pub fn get_mut(self, value: &mut T) -> &mut <T as FieldAccess<FIELD_NAME>>::FieldType {
        value.get_mut()
    }

    pub fn set(self, value: &mut T, field_value: <T as FieldAccess<FIELD_NAME>>::FieldType) {
        value.set(field_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Test {
        age: usize,
        category: &'static str,
        items: Vec<i32>,
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

    impl FieldAccess<2> for Test {
        type FieldType = Vec<i32>;

        fn get(&self) -> &Self::FieldType {
            &self.items
        }

        fn get_mut(&mut self) -> &mut Self::FieldType {
            &mut self.items
        }
    }

    #[derive(Debug, Default)]
    struct TestFields {
        age: Field<Test, 0>,
        category: Field<Test, 1>,
        items: Field<Test, 2>,
    }

    #[test]
    fn test_field_access() {
        let mut value = Test {age: 25, category: "animals", items: Vec::new()};

        let fields = TestFields::default();

        assert_eq!(*fields.age.get(&value), 25);
        assert_eq!(*fields.category.get(&value), "animals");
        assert!(fields.items.get(&value).is_empty());

        fields.age.set(&mut value, 26);

        assert_eq!(*fields.age.get(&value), 26);
        assert_eq!(*fields.category.get(&value), "animals");
        assert!(fields.items.get(&value).is_empty());

        fields.items.get_mut(&mut value).push(32);

        assert_eq!(*fields.age.get(&value), 26);
        assert_eq!(*fields.category.get(&value), "animals");
        assert_eq!(*fields.items.get(&value), vec![32]);
    }
}
