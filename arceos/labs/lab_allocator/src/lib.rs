//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
// use axlog::debug;
use core::alloc::Layout;
use core::ptr::NonNull;

const POOL_SAVE: usize = 43008;
const ITEMS_SAVE: usize = 384;

fn calc(i: usize) -> usize {
    let mut sum = 0;
    let mut base = 32;
    for x in 0..=14 {
        if x % 2 == 1 {
            sum += base + i;
        }
        base *= 2;
    }
    sum
}

pub struct LabByteAllocator {
    start: usize,
    end: usize,

    l_pos: usize,
    r_pos: usize,

    i: usize,           // indicator
    pool_count: usize,  // pool count
    items_count: usize, // items count
    pool_len: usize,
    items_len: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,

            l_pos: 0,
            r_pos: 0,

            i: 0,
            pool_count: 0,
            items_count: 0,
            pool_len: 0,
            items_len: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        // axlog::debug!(
        //     "LabByteAllocator init: [{:#x}, {:#x}) {}",
        //     start,
        //     start + size,
        //     size
        // );
        self.start = start;
        self.end = start + size;
        self.l_pos = start + POOL_SAVE + ITEMS_SAVE;
        self.r_pos = self.l_pos + calc(self.i);
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.end += size;
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        // debug!("LabByteAllocator alloc: {}byte", layout.size());

        let ptr;
        if self.items_count == 15 {
            self.i += 1;
            self.pool_count += 7;
            self.items_count = 0;
            self.items_len = 0;
            self.r_pos = self.l_pos + calc(self.i);

            // extend pool
            if self.pool_count > self.pool_len {
                // debug!("extend pool {} {}", self.pool_len, self.pool_count);
                if self.pool_len == 0 {
                    self.pool_len = 7;
                } else {
                    self.pool_len *= 2;
                }

                ptr = self.start;
                // debug!(
                //     "alloc: [{:#x}, {:#x}) {}byte",
                //     ptr,
                //     ptr + layout.size(),
                //     layout.size()
                // );
                return Ok(NonNull::new(ptr as *mut u8).unwrap());
            }
        }
        // extend items
        if self.items_count > self.items_len {
            // debug!("extend items {} {}", self.items_count, self.items_len);
            if self.items_len == 0 {
                self.items_len = 4;
            } else {
                self.items_len *= 2;
            }

            ptr = self.start + POOL_SAVE;
            // debug!(
            //     "alloc: [{:#x}, {:#x}) {}byte",
            //     ptr,
            //     ptr + layout.size(),
            //     layout.size()
            // );
            return Ok(NonNull::new(ptr as *mut u8).unwrap());
        }

        if self.items_count % 2 == 0 {
            ptr = self.r_pos;
            if ptr + layout.size() >= self.end {
                return Err(AllocError::NoMemory);
            }
            self.r_pos += layout.size();
        } else {
            ptr = self.l_pos;
            if ptr + layout.size() >= self.end {
                return Err(AllocError::NoMemory);
            }
            self.l_pos += layout.size();
        }

        // debug!(
        //     "alloc: [{:#x}, {:#x}) {}byte",
        //     ptr,
        //     ptr + layout.size(),
        //     layout.size()
        // );

        self.items_count += 1;
        return Ok(NonNull::new(ptr as *mut u8).unwrap());
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        // debug!(
        //     "dealloc: [{:#x}, {:#x}) {}byte",
        //     pos.as_ptr() as usize,
        //     pos.as_ptr() as usize + layout.size(),
        //     layout.size()
        // );
    }
    fn total_bytes(&self) -> usize {
        // debug!("total_bytes: {}", self.end - self.start);
        self.end - self.start
    }
    fn used_bytes(&self) -> usize {
        unimplemented!();
    }
    fn available_bytes(&self) -> usize {
        unimplemented!();
    }
}
