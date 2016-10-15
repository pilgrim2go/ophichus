use std::cmp::Ordering;


pub trait SlotValue : Sized + Ord + Default {
    fn combine(Vec<Self>) -> Self;
    fn partial_cmp(&self, other: &Self) -> Ordering {Ordering::Equal}
    fn eq(&self, other: &Self) -> bool {true}
}

trait Slot : Ord {}
