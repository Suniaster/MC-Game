
pub mod vec_storage;

use vec_storage::*;

#[derive(Debug)]
pub struct GenerationalIndex {
    idx: usize,
    generation: usize
}

impl GenerationalIndex{
    #[inline]
    pub fn index(&self)->usize{return self.idx}
}

struct AllocatorEntry {
    is_alive: bool,
    generation: usize
}

pub struct ComponentVecAllocator {
    entries: Vec< AllocatorEntry >,
    free_spaces: Vec<usize>
}

impl ComponentVecAllocator{
    pub fn new() -> ComponentVecAllocator{
        return ComponentVecAllocator{
            entries: vec![], free_spaces: vec![]
        }
    }

    pub fn allocate(&mut self) -> GenerationalIndex{
        let mut index = self.entries.len();
        let mut gen = 0;

        if self.free_spaces.len() > 0 {
            index = self.free_spaces.pop().unwrap();
            gen = self.entries[index].generation + 1;
            self.entries[index] = AllocatorEntry {
                is_alive: true, generation: gen
            };
        }
        else{
            self.entries.push(AllocatorEntry {
                is_alive: true, generation: gen
            });
        }
        return GenerationalIndex {
            idx: index, generation: gen
        }
    }

    pub fn deallocate(&mut self, index: &GenerationalIndex)-> bool{
        if !self.is_live(&index){return false}
        self.entries[index.idx].is_alive = false;
        self.free_spaces.push(index.idx);
        return true;
    }

    #[inline]
    fn is_live(&self, index: &GenerationalIndex) -> bool {
        if index.idx >= self.entries.len() {return false;}
        return self.entries[index.idx].generation == index.generation;
    }
}
