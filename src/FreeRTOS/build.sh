rm -r build/*

bindgen --ctypes-prefix=cty --use-core c_src/wrapper.c -o src/bindings.rs -- -I c_src/include -I c_src/portable/GCC/ARM_CA53_64_RaspberryPi3
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/FreeRTOS_asm_vector.o c_src/FreeRTOS_asm_vector.S
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/startup.o c_src/startup.S
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/portASM.o c_src/portable/GCC/ARM_CA53_64_RaspberryPi3/portASM.S
# aarch64-none-elf-gcc c_src/list.c c_src/queue.c c_src/tasks.c c_src/timers.c c_src/wrapper.c c_src/portable/GCC/ARM_CA53_64_RaspberryPi3/port.c c_src/portable/GCC/ARM_CA53_64_RaspberryPi3/portASM.S c_src/portable/MemMang/heap_1.c c_src/FreeRTOS_asm_vector.S c_src/startup.S -I c_src -I c_src/include -I c_src/portable/GCC/ARM_CA53_64_RaspberryPi3 -mcpu=cortex-a53 -fpic -ffreestanding -O2 -std=gnu11 -T raspberrypi3.ld -o build/wrapper.o
cargo +nightly rustc --target=aarch64-unknown-none -Zbuild-std=panic_abort -- -lc --emit=obj -o build/FreeRTOS.o
cp $(find build/FreeRTOS-*.o) build/FreeRTOS.o

# aarch64-none-elf-gcc -Wl,--build-id=none -std=gnu11 -T raspberrypi3.ld -o build/FreeRTOS.elf -ffreestanding -O2 -nostdlib build/FreeRTOS_asm_vector.o build/startup.o build/portASM.o build/wrapper.o build/FreeRTOS.o
