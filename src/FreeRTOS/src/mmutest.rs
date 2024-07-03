use crate::linkedlist::*;
use crate::mmuconf::*;

static mut seed: u32 = 114514;

fn gen_rand() -> u32 {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    seed
}

fn random_initialize(l: u32, r: u32) {
    for i in 0..disk_size {
        disk[i] = gen_rand() % (r - l + 1) + l;
    }
}

fn swap(i: i32, j: i32) {
    let tmp = read_memory(i); // int tmp = mem[i];
    write_memory(i, read_memory(j)); // mem[i] = mem[j];
    write_memory(j, tmp); // mem[j] = tmp;
}

fn bubble_sort() {
    for i in 0..disk_size - 1 {
        for j in 0..disk_size - i - 1 {
            if read_memory(j) > read_memory(j + 1) {
                swap(j, j + 1);
            }
        }
    }
}
