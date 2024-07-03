rm -r build/*

bindgen --ctypes-prefix=cty --use-core c_src/wrapper.c -o src/bindings.rs -- -I c_src/include -I c_src/portable/GCC/ARM_CA53_64_RaspberryPi3
cargo +nightly rustc --target=aarch64-unknown-none -Zbuild-std=panic_abort -- -lc --emit=obj -o build/FreeRTOS.o
cp $(find build/FreeRTOS-*.o) build/FreeRTOS.o

# aarch64-none-elf-gcc -Wl,--build-id=none -std=gnu11 -T raspberrypi3.ld -o build/FreeRTOS.elf -ffreestanding -O2 -nostdlib build/FreeRTOS_asm_vector.o build/startup.o build/portASM.o build/wrapper.o build/FreeRTOS.o
aarch64-none-elf-objcopy build/FreeRTOS.elf -O binary build/kernel.img
