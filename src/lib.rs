mod field;
mod expr;

pub use field::*;
pub use expr::*;

pub mod types;

pub trait Record: Sized {
    type PrimaryKey;
    type Insert;

    fn create_primary_key(key: usize) -> Self::PrimaryKey;
    fn from_insert(record: Self::Insert, primary_key: Self::PrimaryKey) -> Self;
}

pub struct Table<R: Record> {
    records: Vec<R>,
    next_primary_key: usize,
}

impl<R: Record> Default for Table<R> {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            next_primary_key: 0,
        }
    }
}

impl<R: Record> Table<R> {
    pub fn all(&self) -> TableIter<R> {
        TableIter {records: &self.records}
    }

    pub fn insert(&mut self, record: R::Insert) {
        let next_key = self.generate_next_primary_key();
        let primary_key = R::create_primary_key(next_key);
        let record = R::from_insert(record, primary_key);
        self.records.push(record);
    }

    fn generate_next_primary_key(&mut self) -> usize {
        let key = self.next_primary_key;
        self.next_primary_key += 1;
        key
    }
}

pub struct TableIter<'a, R: Record> {
    records: &'a [R],
}

impl<'a, R: Record> Iterator for TableIter<'a, R> {
    type Item = &'a R;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.records.iter().size_hint()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.records.get(0)?;
        self.records = &self.records[1..];
        Some(item)
    }
}

impl<'a, R: Record> ExactSizeIterator for TableIter<'a, R> {}
