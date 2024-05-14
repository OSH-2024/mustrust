extern crate cc;

fn main() {
	cc::Build::new()
        .file("src/wrapper.c")
        .compiler("aarch64-none-elf-gcc")
        .flag("-I").flag("src")
        .flag("-I").flag("src/include")
        .flag("-I").flag("src/portable/GCC/ARM_CA53_64_RaspberryPi3")
        .flag("-mcpu=cortex-a53")
        .flag("-fpic")
        .flag("-ffreestanding")
        .flag("-O2")
        .flag("-std=gnu99")
        .target("aarch64-none-elf")
        .compile("libwrapper.a");
}
