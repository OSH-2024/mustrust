rm build/*

bindgen --ctypes-prefix=cty --use-core c_src/wrapper.c -o src/bindings.rs -- -I c_src/include -I c_src/portable/GCC/ARM_CA53_64_RaspberryPi3
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/FreeRTOS_asm_vector.o c_src/FreeRTOS_asm_vector.S
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/startup.o c_src/startup.S
aarch64-none-elf-as -mcpu=cortex-a53 -c -o build/portASM.o c_src/portable/GCC/ARM_CA53_64_RaspberryPi3/portASM.S
cargo +nightly rustc --target=aarch64-unknown-none -Zbuild-std=panic_abort -- -lc --emit=obj -o build/FreeRTOS.o
cp $(find build/FreeRTOS-*.o) build/FreeRTOS.o

aarch64-none-elf-gcc -Wl,--build-id=none -std=gnu11 -T raspberrypi3.ld -o build/FreeRTOS.elf -ffreestanding -O2 -nostdlib build/FreeRTOS_asm_vector.o build/startup.o build/portASM.o build/wrapper.o build/FreeRTOS.o
