extern crate cc;

fn main() {
	cc::Build::new()
        .file("c_src/list.c")
        .file("c_src/queue.c")
        .file("c_src/tasks.c")
        .file("c_src/timers.c")
        .file("c_src/wrapper.c")
        .file("c_src/portable/MemMang/heap_1.c")
        .file("c_src/portable/GCC/ARM_CA53_64_RaspberryPi3/port.c")
        .file("c_src/portable/GCC/ARM_CA53_64_RaspberryPi3/portASM.S")
        .file("c_src/FreeRTOS_asm_vector.S")
        .file("c_src/startup.S")
        .compiler("aarch64-none-elf-gcc")
        .flag("-I").flag("c_src")
        .flag("-I").flag("c_src/include")
        .flag("-I").flag("c_src/portable/GCC/ARM_CA53_64_RaspberryPi3")
        .flag("-mcpu=cortex-a53")
        .flag("-fpic")
        .flag("-ffreestanding")
        .flag("-O2")
        .flag("-std=gnu11")
        .target("aarch64-none-elf")
        .flag("-T").flag("raspberrypi3.ld")
        .compile("wrapper");
        println!("cargo:rustc-link-search=wrapper");
}
