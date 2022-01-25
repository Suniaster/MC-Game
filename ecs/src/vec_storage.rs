use super::*;

pub struct ArrayEntry<T> {
    pub value: T,
    #[allow(dead_code)]
    generation: usize,
}

// An associative array from GenerationalIndex to some Value T.
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> std::default::Default for GenerationalIndexArray<T>{
    fn default() -> Self {Self(vec![])}
}

impl<T> GenerationalIndexArray<T> {
    pub fn new()->GenerationalIndexArray<T>{
        return GenerationalIndexArray(vec![]);
    }

    // Gets the value for some generational index, the generation must match.
    pub fn remove(&mut self, index: &GenerationalIndex){ 
        if index.idx > self.0.len() {return;}
        self.0[index.idx] = None;
    }

    // Set the value for some generational index.  May overwrite past generation
    // values.
    pub fn set(&mut self, index: &GenerationalIndex, value: T) { 
        while index.idx >= self.0.len() { self.0.push(None) }

        self.0[index.idx] = Some(ArrayEntry {
            value, generation: index.generation
        });   
    }

    // Gets the value for some generational index, the generation must match.
    pub fn get(&self, index: &GenerationalIndex) -> Option<&T> { 
        if index.idx > self.0.len() {return None}
        match &self.0[index.idx] {
            Some(x) => Some(&x.value),
            None => None
        }
    }
    pub fn get_mut(&mut self, index: &GenerationalIndex) -> Option<&mut T> { 
        if index.idx > self.0.len() {return None}
        match &mut self.0[index.idx] {
            Some(x) => Some(&mut x.value),
            None => None
        }
    }

    pub fn data(&self)-> &Vec<Option<ArrayEntry<T>>>{
        return &self.0;
    }

    pub fn data_mut(&mut self)-> &mut Vec<Option<ArrayEntry<T>>>{
        return &mut self.0;
    }

    // pub fn iter(&self)
}