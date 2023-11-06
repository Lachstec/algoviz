use super::SortingAlgorithm;
use crate::visualizer::Array;

pub struct BubbleSort;

impl SortingAlgorithm for BubbleSort {
   fn name(&self) -> String {
       "Bubblesort".to_string()
   }

   fn sort(&self, array: &mut Array) {
        let len = array.len();
        for i in (0..len - 1).rev() {
            for j in 0..i - 1 {
                if array.value_at(j) > array.value_at(j + 1) {
                    array.swap(j, j + 1)
                }
            } 
        } 
   }
}
