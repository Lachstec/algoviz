use crate::visualizer::Array;

pub trait SortingAlgorithm {
    fn sort(&self, array: &mut Array);
    fn name() -> &'static str;
}

pub mod bubble_sort;
