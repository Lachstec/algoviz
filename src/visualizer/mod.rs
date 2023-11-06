use crate::sorting::SortingAlgorithm;

mod array;
mod drawing;

pub use array::{ArrayOrder, Array};

pub struct Visualizer {
    arr_size: usize,
    minimum: u32,
    maximum: u32,
    width: usize,
    height: usize,
    algorithm: Box<dyn SortingAlgorithm>,
    order: ArrayOrder,
    arr: Array,
}

impl Visualizer {
    pub fn builder() -> VisualizerBuilder {
       VisualizerBuilder::default() 
    }
}

impl std::convert::From<VisualizerBuilder> for Visualizer {
   fn from(value: VisualizerBuilder) -> Self {
        let arr_size = value.arr_size.unwrap_or(100);
        let min = value.minimum.unwrap_or(1);
        let max = value.maximum.unwrap_or(100);
        let width = value.width.unwrap_or(800);
        let height = value.width.unwrap_or(600);
        let order = value.order.unwrap_or(ArrayOrder::Unordered);
        let arr = Array::new(arr_size, min, max, order);

        Visualizer {
            arr_size,
            minimum: min,
            maximum: max,
            width,            
            height,
            algorithm: value.algorithm,
            order,
            arr 
        }

   }
}

pub struct VisualizerBuilder {
    arr_size: Option<usize>,
    minimum: Option<u32>,
    maximum: Option<u32>,
    width: Option<usize>,
    height: Option<usize>,
    order: Option<ArrayOrder>,
    algorithm: Box<dyn SortingAlgorithm>
}

impl VisualizerBuilder {
    pub fn set_arr_size(mut self, size: usize) -> Self {
        self.arr_size = Some(size);
        self
    }
    
    pub fn set_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn set_height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn set_order(mut self, order: ArrayOrder) -> Self {
        self.order = Some(order);
        self
    }

    pub fn set_maximum(mut self, maximum: u32) -> Self {
        self.maximum = Some(maximum);
        self
    }

    pub fn set_minimum(mut self, minimum: u32) -> Self {
        self.minimum = Some(minimum);
        self
    }

    pub fn build(mut self) -> Visualizer {
        self.into()
    }
}

impl std::default::Default for VisualizerBuilder {
    fn default() -> Self {
        Self{arr_size: None, minimum: None, maximum: None, width: None, height: None, order: None, algorithm: Box::new(crate::sorting::BubbleSort)}
    }
}
