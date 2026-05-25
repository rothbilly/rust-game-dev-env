use std::hash::Hash;
use std::collections::HashMap;





#[derive(PartialEq, Eq, Clone)]
struct InspectableData<T>
where T: Hash + PartialEq + Eq + Clone,
{
    data: HashMap<String, T>
}

impl<T> InspectableData<T>
where T: Hash + PartialEq + Eq + Clone,
{
    
    fn new() -> Self{
        Self{ data: HashMap::new()}
    }

    fn new_with(data: HashMap<String, T>) -> Self {
        Self { data }
    }
    
    pub fn add_value(&mut self, _key: String, _value: T){

    }
}