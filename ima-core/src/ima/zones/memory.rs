/// Created by Virgile HENRY, 2023/09/28

use std::fmt::Display;

use crate::ima::{data_type::DataType, error::ImaExecutionError};

pub mod allocator;

/// Pointer to the stack of the IMA machine.
/// It is only stored on 31 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackPointer(u32);

impl StackPointer {
    /// Returns a new stack pointer pointing to the first element of the stack.
    pub fn zero() -> StackPointer {
        StackPointer(0)
    }

    /// Returns a new stack pointer with the given offset.
    /// If this fails, the offset caused an overflow.
    pub fn offset(self, offset: i32) -> Option<StackPointer> {
        // safe because the first bit is always 0
        let ptr = i32::try_from(self.0).unwrap();
        u32::try_from(ptr + offset).ok()
            .and_then(|value| {
                if value & 0x8000_0000 == 0 { Some(StackPointer(value)) }
                else { None } // overflowed to heap
            })
    }

    /// Get the stack pointer as an index, to get values from the stack.
    pub fn as_index(self) -> usize {
        self.0 as usize
    }
}

impl Display for StackPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@ Stack {}", self.0)
    }
}

/// Pointer to the heap of the IMA machine.
/// It is only stored on 31 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeapPointer(u32);

impl HeapPointer {
    /// Returns a new heap pointer with the given offset.
    /// If this fails, the offset caused an overflow.
    pub fn offset(self, offset: i32) -> Option<HeapPointer> {
        // safe because the first bit is always 0
        let ptr = i32::try_from(self.0).unwrap();
        u32::try_from(ptr + offset).ok()
            .and_then(|value| {
                if value & 0x8000_0000 == 0 { Some(HeapPointer(value)) }
                else { None } // overflowed to heap
            })
    }

    /// Get the heap pointer as an index, to get values from the heap.
    pub fn as_index(self) -> usize {
        self.0 as usize
    }
}

impl Display for HeapPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@ Heap {}", self.0)
    }
}

/// Pointer type of the IMA machine.
/// The inner types are u32, but the first bit is kept for the type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pointer {
    Stack(StackPointer),
    Heap(HeapPointer),
    Null,
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Pointer::Stack(ptr) => write!(f, "{}", ptr),
            Pointer::Heap(ptr) => write!(f, "{}", ptr),
            Pointer::Null => write!(f, "Null"),
        }
    }
}

impl From<u32> for Pointer {
    fn from(value: u32) -> Self {
        if value & 0x8000_0000 == 0 { Pointer::Stack(StackPointer(value)) }
        else { Pointer::Heap(HeapPointer(value & 0x7FFF_FFFF)) }
    }
}

impl Pointer {
    /// Offset the given pointer. If this fails, the offset caused an overflow.
    pub fn offset(self, offset: i32) -> Result<Pointer, ImaExecutionError> {
        match self {
            Pointer::Stack(ptr) => match ptr.offset(offset) {
                Some(ptr) => Ok(Pointer::Stack(ptr)),
                None => Err(ImaExecutionError::InvalidMemoryAddress(self)),
            }
            Pointer::Heap(ptr) => match ptr.offset(offset) {
                Some(ptr) => Ok(Pointer::Heap(ptr)),
                None => Err(ImaExecutionError::InvalidMemoryAddress(self)),
            }
            Pointer::Null => Err(ImaExecutionError::InvalidMemoryAddress(self)),
        }
    }
}

/// Memory of the IMA machine, split in stack and heap.
/// Accessible via the pointer type. The total address mode is 32 bits,
/// with the first bit being the type of memory (stack or heap).
#[cfg(not(feature = "public-ima"))]
pub struct Memory {
    /// Stack of the IMA memory.
    stack: Vec<DataType>,
    /// Heap of the IMA memory.
    heap: Vec<Option<DataType>>,
    /// Allocator for the heap.
    allocator: Box<dyn allocator::Allocator>,
}

#[cfg(feature = "public-ima")]
pub struct Memory {
    /// Stack of the IMA memory.
    pub stack: Vec<DataType>,
    /// Heap of the IMA memory.
    pub heap: Vec<Option<DataType>>,
    /// Allocator for the heap.
    pub allocator: Box<dyn allocator::Allocator>,
}

impl Memory {
    /// Create a new memory with the given sizes for the stack and heap.
    pub fn new(heap_size: usize, stack_size: usize) -> Memory {
        Memory {
            stack: vec![DataType::Undefined; stack_size],
            heap: vec![None; heap_size],
            allocator: Box::new(allocator::LinearAllocator::new()),
        }
    }

    /// Access a word in memory at the given pointer.
    pub fn get(&self, at: Pointer) -> Option<DataType> {
        match at {
            Pointer::Stack(index) => self.get_stack(index),
            Pointer::Heap(index) => self.get_heap(index),
            Pointer::Null => None,
        }
    }

    /// Access a word on the stack with the given stack pointer.
    pub fn get_stack(&self, at: StackPointer) -> Option<DataType> {
        self.stack.get(at.as_index()).map(|x| *x)
    }

    /// Access a word on the heap with the given heap pointer.
    pub fn get_heap(&self, at: HeapPointer) -> Option<DataType> {
        self.heap.get(at.as_index()).map(|x| *x)?
    }

    /// Set a word in memory at the given pointer.
    pub fn set(&mut self, at: Pointer, value: DataType) -> Result<(), ImaExecutionError> {
        match at {
            Pointer::Stack(index) => self.set_stack(index, value),
            Pointer::Heap(index) => self.set_heap(index, value),
            Pointer::Null => Err(ImaExecutionError::InvalidMemoryAddress(at)),
        }
    }

    /// Set a word on the stack with the given stack pointer.
    pub fn set_stack(&mut self, at: StackPointer, value: DataType) -> Result<(), ImaExecutionError> {
        match self.stack.get_mut(at.as_index()) {
            Some(x) => { *x = value; Ok(()) },
            None => Err(ImaExecutionError::StackOverflow),
        }
    }

    /// Set a word on the heap with the given heap pointer.
    pub fn set_heap(&mut self, at: HeapPointer, value: DataType) -> Result<(), ImaExecutionError> {
        match self.heap.get_mut(at.as_index()) {
            Some(x) => { *x = Some(value); Ok(()) },
            None => Err(ImaExecutionError::InvalidMemoryAddress(Pointer::Heap(at))),
        }
    }

    /// Get the size of the stack.
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Allocate a new block of memory on the heap and returns a pointer to it.
    /// If the heap is full, this will fail and return None. 
    pub fn allocate(&mut self, size: usize) -> Option<HeapPointer> {
        self.allocator.allocate(&mut self.heap, size)
    }

    /// Free a block of memory on the heap.
    /// If The pointer does not point to a valid allocation, this will fail and return None. 
    pub fn free(&mut self, ptr: HeapPointer) -> Option<()> {
        self.allocator.free(&mut self.heap, ptr)
    }

    /// Clear the memory.
    pub fn clear(&mut self) {
        self.stack.iter_mut().for_each(|v| *v = DataType::Undefined);
        self.heap.iter_mut().for_each(|v| *v = None);
    }

    /// Display the stack to the given output between start and end.
    pub fn display_stack(&mut self, start:u32, end:u32, output: &mut impl std::io::Write, sp: StackPointer) -> Result<(), std::io::Error> {
        for i in (start..=end.min(self.stack.len() as u32)).rev() {
            let sp = if sp == StackPointer(i) { "SP ->" } else { "     " };
            writeln!(output, "{} {:<3}| {}", sp, i, self.stack[i as usize])?;
        }
        Ok(())
    }

    /// Look for an allocated block of memory containing the pointer, and display the block to the given output. 
    pub fn display_block(&mut self, pointer: HeapPointer, output: &mut impl std::io::Write, register: u8) -> Result<(), std::io::Error> {
        let block = self.allocator.get_block(pointer);
        match block {
            Some((start, size)) => {
                writeln!(output, "Block at {} of size {}:", start, size)?;
                for i in start.0..(start.0 + size as u32) {
                    let reg = if pointer.0 == i { format!("R{register} -> ") } else { "      ".to_string() };
                    writeln!(output, "{reg}{:<3}| {}", i, self.heap[i as usize].unwrap())?;
                }
            },
            None => writeln!(output, "Invalid pointer {}", pointer)?,
        }
        Ok(())
    }
}