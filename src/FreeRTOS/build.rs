extern crate cc;

fn main() {
	cc::Build::new()
        .file("src/wrapper.c")
        .flag("-I").flag("src")
        .flag("-I").flag("src/include")
        .flag("-I").flag("src/portable/GCC/ARM_CA53_64_RaspberryPi3")
        .target("aarch64-none-elf")
        .compile("libwrapper.a");
}
