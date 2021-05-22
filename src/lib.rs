pub mod types;

use std::marker::PhantomData;

pub trait Record {
}

pub struct Table<R: Record> {
    _marker: PhantomData<R>,
}
