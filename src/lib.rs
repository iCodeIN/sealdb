pub mod types;

use std::marker::PhantomData;

pub trait Record {
}

pub struct Table<R: Record> {
    _marker: PhantomData<R>,
}

impl<R: Record> Default for Table<R> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
