use crate::visualizer::Array;

pub trait SortingAlgorithm {
    fn sort(&self, array: &mut Array);
    fn name(&self) -> String;
}

mod bubble_sort;

pub use bubble_sort::BubbleSort;
