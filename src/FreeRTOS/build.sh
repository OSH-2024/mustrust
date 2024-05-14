bindgen src/wrapper.c -o src/bindings.rs -- -I src/include -I src/portable/GCC/ARM_CA53_64_RaspberryPi3
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/FreeRTOS_asm_vector.o src/FreeRTOS_asm_vector.S
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/startup.o src/startup.S
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/portASM.o src/portable/GCC/ARM_CA53_64_RaspberryPi3/portASM.S
cargo rustc -- --emit=obj

# aarch64-none-elf-gcc -Wl,--build-id=none -std=gnu11 -T raspberrypi3.ld -o $@ -ffreestanding -O2 -nostdlib build/FreeRTOS_asm_vector.o build/startup.o build/portASM.o
