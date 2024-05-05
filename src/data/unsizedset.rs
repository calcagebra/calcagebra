#[derive(Debug)]
pub enum BaseSet {
    Natural,
    Whole,
    Integers,
    Real,
}

#[derive(Debug)]
pub struct UnsizedSet {
    upper_limit: f32,
    lower_limit: f32,
    base_set: BaseSet,
}

impl UnsizedSet {
    pub fn new() {
        
    }
}