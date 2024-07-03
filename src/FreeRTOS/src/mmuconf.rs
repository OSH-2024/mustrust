use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;
use crate::linkedlist::*;

#[derive(Default)]
pub struct Line {
    valid: bool,
    reference: bool,
    modified: bool,
    page_number: i32,
    frame_number: i32,
}

#[derive(Default)]
pub struct Entry {
    valid: bool,
    modified: bool,
    frame_number: i32,
    disk_address: i32,
}

#[derive(Default)]
pub struct TCB {
    name: String,
    page_table: Vec<Entry>,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            valid: false,
            reference: false,
            modified: false,
            page_number: 0,
            frame_number: 0,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            valid: false,
            modified: false,
            frame_number: 0,
            disk_address: 0,
        }
    }
}

impl Default for TCB {
    fn default() -> Self {
        Self {
            name: String::new(),
            page_table: Vec::new(),
        }
    }
}

pub const replace_use_lru: bool = true;
pub const memory_size_kb: i32 = 8;

pub const page_size: i32 = 128 * memory_size_kb;
pub const memory_size: i32 = 1024 * memory_size_kb;
pub const page_count: i32 = memory_size / page_size;
pub const disk_size: i32 = 4096 * memory_size_kb; // Assuming 4 times memory size
pub const virtual_space: i32 = 2048 * memory_size_kb; // Assuming 2 times memory size
pub const page_table_size: i32 = virtual_space / page_size;
pub const tlb_size: i32 = 4 * memory_size_kb;
pub const time_tlb_access: u64 = 1;
pub const time_memory_access: u64 = 100;
pub const time_cache_access: u64 = 100;
pub const time_disk_access: u64 = 1000000;
pub const start_address: i32 = 0;

pub static mut memory: [u32; memory_size] = [0; memory_size];
pub static mut disk: [u32; disk_size] = [0; disk_size];
pub static mut tlb: [Line; tlb_size] = [0; tlb_size];
pub static mut tcb: Box<TCB> = Box::new(TCB::default());

pub static mut tlb_hit: u32 = 0;
pub static mut tlb_miss: u32 = 0;
pub static mut memory_hit: u32 = 0;
pub static mut memory_miss: u32 = 0;
pub static mut replace_count_fifo: u32 = 0;
pub static mut time_cost: u64 = 0;

pub fn stat_init() {
    tlb_hit = 0;
    tlb_miss = 0;
    memory_hit = 0;
    memory_miss = 0;
    replace_count_fifo = 0;
    time_cost = 0.0;
}

#[derive(Default, PartialEq)]
pub struct Item {
    frame_number: i32,
    page_number: i32,
    task_belong: Box<TCB>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            frame_number: 0,
            page_number: 0,
            task_belong: Box::new(TCB::default()),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.frame_number == other.frame_number
    }
}

static mut lru_list: Linkedlist<Item> = Linkedlist::<Item>::new();
static mut fifo_list: Linkedlist<Item> = Linkedlist::<Item>::new();

pub fn read_to_memory(frame: i32, disk_address_begin: i32) {
    for i in 0..page_size {
        let memory_address = frame * page_size + i;
        let disk_address = disk_address_begin + i;
        memory[memory_address as usize] = disk[disk_address as usize];
    }
    time_cost += time_disk_access;
}

pub fn write_to_disk(frame: i32, disk_address_begin: i32) {
    for i in 0..page_size {
        let memory_address = frame * page_size + i;
        let disk_address = disk_address_begin + i;
        disk[disk_address as usize] = memory[memory_address as usize];
    }
    time_cost += time_disk_access;
}

pub fn page_table_init() {
    tcb.page_table.resize(page_table_size as usize, Entry::default());
    for i in 0..page_table_size {
        tcb.page_table[i as usize].disk_address = start_address + i * page_size;
    }
    for i in 0..page_count {
        read_to_memory(i, start_address + i * page_size);
        tcb.page_table[i as usize].frame_number = i;
        tcb.page_table[i as usize].valid = true;
    }
}

pub fn address_map(vaddr: i32, write: bool) -> i32 {
    let page_number = vaddr / page_size;
    let offset = vaddr % page_size;
    let physical_address = tlb_search(vaddr, write);

    if physical_address != -1 {
        tlb_hit += 1;
        memory_hit += 1;
        return physical_address;
    }

    tlb_miss += 1;
    time_cost += time_memory_access;
    if !tcb.page_table[page_number as usize].valid {
        page_fault(&mut tcb.page_table[page_number as usize], page_number);
        memory_miss += 1;
    }
    else {
        memory_hit += 1;
    }
    let physical_address = tcb.page_table[page_number as usize].frame_number * page_size + offset;
    if replace_use_lru {
        lru_list.move_to_peek(&tcb.page_table[page_number as usize]);
    }
    if write {
        tcb.page_table[page_number as usize].modified = true;
    }
    tlb_update(page_number, tcb.page_table[page_number as usize].frame_number);
    physical_address
}

pub fn page_fault(page: &mut Entry, page_number: i32) {
    let end_node: &mut Item = if replace_use_lru {
        lru_list.last_mut().unwrap()
    }
    else {
        fifo_list.peek_mut().unwrap()
    };
    
    if end_node.task_belong == tcb {
        for i in 0..tlb_size {
            if tlb[i].valid && tlb[i].page_number == end_node.page_number {
                tlb[i].valid = false;
                if tlb[i].modified {
                    tcb.page_table[end_node.page_number as usize].modified = true;
                }
                break;
            }
        }
    }

    if end_node.task_belong.page_table[end_node.page_number as usize].modified {
        let disk_address_out = end_node.task_belong.page_table[end_node.page_number as usize].disk_address;
        write_to_disk(end_node.frame_number, disk_address_out);
    }
    end_node.task_belong.page_table[end_node.page_number as usize].valid = false;

    let disk_address_in = page.disk_address;
    read_to_memory(page.frame_number, disk_address_in);
    page.valid = true;
    page.modified = false;
    page.frame_number = end_node.frame_number;
    end_node.task_belong = tcb;
    end_node.page_number = page_number;
    if !replace_use_lru {
        fifo_list.pop();
    }
}

pub fn lru_list_init() {
    lru_list = Linkedlist::new();
    for i in 0..page_count {
        let mut s = Item {
            task_belong: tcb,
            page_number: i,
            frame_number: i,
        };
        lru_list.push(&mut s);
    }
}

pub fn read_memory(vaddr: i32) {
    let physical_address = address_map(vaddr, false);
    let data = memory[physical_address as usize];
    time_cost += time_cache_access;
    data
}

pub fn write_memory(vaddr: i32, data: i32) {
    let physical_address = address_map(vaddr, true);
    memory[physical_address as usize] = data;
    time_cost += time_cache_access;
}