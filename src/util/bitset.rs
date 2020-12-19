#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct SmallIntegerSet64 {
    data: u64,
}
impl SmallIntegerSet64 {
    pub fn new() -> SmallIntegerSet64 {
        SmallIntegerSet64 {
            data: 0,
        }
    }
    pub fn new_filled() -> SmallIntegerSet64 {
        SmallIntegerSet64 {
            data: std::u64::MAX,
        }
    }
    pub fn insert(&mut self, i: usize) {
        self.data |= 1 << (i & 63);
    }
    pub fn remove(&mut self, i: &usize) {
        self.data &= !(1 << (i & 63));
    }
    pub fn contains(&self, i: usize) -> bool {
        (self.data >> (i & 63)) & 1 == 1
    }
    pub fn empty(&self) -> bool {
        self.data == 0
    }
    pub fn len(&self) -> usize {
        self.data.count_ones() as usize
    }
    pub fn retain_intersection(&mut self, other: &SmallIntegerSet64) {
        self.data &= other.data;
    }
    pub fn pop(&mut self) -> Option<usize> {
        let trailing_zeros = self.data.trailing_zeros() as usize;
        if trailing_zeros < 64 {
            self.remove(&trailing_zeros);
            return Some(trailing_zeros);
        }
        None
    }

}