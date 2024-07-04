use crate::bindings::*;
use crate::uart::*;

static mut seed: u32 = 1919810;
const times: u32 = 8;
const disk_size: u32 = 4096 * times;
const virtual_space: u32 = 2048 * times;

fn gen_rand() -> u32 {
    unsafe {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    }
}

pub fn random_initialize(l: u32, r: u32) {
    unsafe {
        for i in 0..disk_size {
            disk[i as usize] = (gen_rand() % (r - l + 1) + l) as i32;
        }
    }
}

fn swap(i: i32, j: i32) {
    unsafe {
        let tmp = read_memory(i); // int tmp = mem[i];
        write_memory(i, read_memory(j)); // mem[i] = mem[j];
        write_memory(j, tmp); // mem[j] = tmp;
    }
}

pub fn bubble_sort() {
    unsafe {
        for i in 0..virtual_space - 1 {
            for j in 0..virtual_space - i - 1 {
                if read_memory(j as i32) > read_memory(j as i32 + 1) {
                    swap(j as i32, j as i32 + 1);
                }
            }
        }
    }
}
