/// Created by Virgile HENRY, 2023/09/28

use std::collections::HashMap;

use crate::ima::data_type::DataType;

use super::HeapPointer;


pub trait Allocator {
    fn allocate(&mut self, memory: &mut [Option<DataType>], size: usize) -> Option<HeapPointer>;
    fn free(&mut self, memory: &mut [Option<DataType>], ptr: HeapPointer) -> Option<()>;
    fn get_block(&self, ptr: HeapPointer) -> Option<(HeapPointer, usize)>;
}


/// Super naive linear allocator. Finds the next spot in memory big enough to fit the allocation.
pub struct LinearAllocator {
    allocations: HashMap<HeapPointer, usize>,
}

impl LinearAllocator {
    pub fn new() -> LinearAllocator {
        LinearAllocator {
            allocations: HashMap::new(),
        }
    }
}

impl Allocator for LinearAllocator {
    fn allocate(&mut self, memory: &mut [Option<DataType>], size: usize) -> Option<HeapPointer> {
        let mut ptr = HeapPointer(0);
        let mut available_size = 0;
        loop {
            if size == available_size {
                for i in ptr.as_index()..(ptr.offset(size as i32).unwrap().as_index()) {
                    memory[i] = Some(DataType::Undefined);
                }
                self.allocations.insert(ptr, size);
                return Some(ptr);
            }

            match memory.get(ptr.0 as usize + available_size) {
                Some(None) => available_size += 1,
                Some(Some(_)) => {
                    ptr = match ptr.offset(available_size as i32 + 1) {
                        Some(ptr) => ptr,
                        None => return None,
                    };
                    available_size = 0;
                },
                None => return None,
            }
        }
    }

    fn free(&mut self, memory: &mut [Option<DataType>], ptr: HeapPointer) -> Option<()> {
        match self.allocations.remove(&ptr) {
            Some(size) => {
                // if the allocation is in the map, we can assume that it is valid
                for i in ptr.as_index()..(ptr.offset(size as i32).unwrap().as_index()) {
                    memory[i] = None;
                }
                Some(())
            },
            None => None,
        }
    }

    fn get_block(&self, ptr: HeapPointer) -> Option<(HeapPointer, usize)> {
        for (start, size) in self.allocations.iter() {
            if ptr.as_index() >= start.as_index() && ptr.as_index() < start.as_index() + size {
                return Some((*start, *size));
            }
        }
        None
    }
}