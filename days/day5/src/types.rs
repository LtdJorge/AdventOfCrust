use std::cmp::Ordering;

pub type LowerNumbers = Vec<usize>;
pub type HigherNumbers = Vec<usize>;

#[derive(Debug)]
pub struct OrderedNumeric<'a> {
    pub(super) numeric: usize,
    pub(super) stored_orders: &'a (LowerNumbers, HigherNumbers),
}

impl PartialEq<Self> for OrderedNumeric<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.numeric == other.numeric
    }
}

impl Eq for OrderedNumeric<'_> {}

impl PartialOrd<Self> for OrderedNumeric<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedNumeric<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_numeric = other.numeric;
        if self.stored_orders.0.iter().any(|&num| num == other_numeric) {
            Ordering::Greater
        } else if self.stored_orders.1.iter().any(|&num| num == other_numeric) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl<'a> OrderedNumeric<'a> {
    pub fn new(numeric: usize, stored_orders: &'a (LowerNumbers, HigherNumbers)) -> Self {
        OrderedNumeric {
            numeric,
            stored_orders,
        }
    }
}

#[derive(Debug)]
pub enum LineType {
    PageOrderingRule(PageOrderingRule),
    Update(Update),
}

#[derive(Debug)]
pub struct PageOrderingRule {
    pub(super) left: usize,
    pub(super) right: usize,
}

#[derive(Debug)]
pub struct Update {
    pub(super) list: Vec<usize>,
}
