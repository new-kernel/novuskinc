use core::borrow::Borrow;
use core::cell::Cell;

/// A kernel memory region, contains the start and end of the memory.
#[cfg(target_pointer_width = "64")]
pub type KernelMemoryRegion = [(u64, u64); 16];

/// A kernel memory region, contains the start and end of the memory.
#[cfg(target_pointer_width = "32")]
pub type KernelMemoryRegion = [(u32, u32); 16];

pub struct KernelMemory {
    #[cfg(target_pointer_width = "64")]
    pub memory: Cell<KernelMemoryRegion>,

    #[cfg(target_pointer_width = "32")]
    pub memory: Cell<KernelMemoryRegion>,
}

impl KernelMemory {
    pub fn empty() -> Self {
        return KernelMemory {
            memory: Cell::new([(0, 0); 16]),
        };
    }

    #[cfg(target_pointer_width = "64")]
    /// Returns the size of the added memory
    pub fn add_memory(&mut self, start: u64, end: u64) -> Result<u64, &'static str> {
        if end > start {
            return Err("End of memory cannot be larger than start");
        }

        let mut memory = self.memory.get();

        for i in 0..memory.len() {
            if memory[i].0 == 0 && memory[i].1 == 0 {
                memory[i].0 = start;
                memory[i].1 = end;
                self.memory.replace(memory);

                return Ok(end - start);
            }
        }

        return Err("Cannot add anymore memory")
    }

    #[cfg(target_pointer_width = "32")]
    /// Returns the size of the added memory
    pub fn add_memory(&mut self, start: u32, end: u32) -> Result<u32, &'static str> {
        let mut memory = self.memory.get();

        for i in 0..memory.len() {
            if memory[i].0 == 0 && memory[i].1 == 0 {
                memory[i].0 = start;
                memory[i].1 = end;
                self.memory.replace(memory);

                return Ok(end - start);
            }
        }

        return Err("Cannot add anymore memory")
    }

    #[cfg(target_pointer_width = "64")]
    /// Take a memory region. This function removes the region from it's list, so that memory region
    /// can't be used again unless you add it back properly (the leftover memory)
    pub fn take(&mut self) -> Result<(u64, u64), &'static str> {
        let mut memory = self.memory.get();

        for i in 0..memory.len() {
            if memory[i].0 != 0 && memory[i].1 != 0 {
                let ret = (memory[i].0, memory[i].1);
                memory[i].0 = 0;
                memory[i].1 = 0;

                self.memory.replace(memory);

                return Ok((memory[i].0, memory[i].1));
            }
        }

        return Err("No memory available to take");
    }


    #[cfg(target_pointer_width = "32")]
    /// Take a memory region. This function removes the region from it's list, so that memory region
    /// can't be used again unless you add it back properly (the leftover memory)
    pub fn take(&mut self) -> Result<(u32, u32), &'static str> {
        let mut memory = self.memory.get();

        for i in 0..memory.len() {
            if memory[i].0 != 0 && memory[i].1 != 0 {
                let ret = (memory[i].0, memory[i].1);
                memory[i].0 = 0;
                memory[i].1 = 0;

                self.memory.replace(memory);

                return Ok((memory[i].0, memory[i].1));
            }
        }

        return Err("No memory available to take");
    }

    pub fn available(&self) -> *const KernelMemoryRegion {
        let mem = &self.memory;
        return &mem.get() as *const KernelMemoryRegion
    }
}
