给FreeRTOS添加MMU支持的可行性分析

## MMU(未完)

MMU的基本目标：

- MMU将虚拟地址(VA)转换为物理地址(PA)，给上层的程序提供抽象。
- 为进程动态分配内存，同时为内存设立边界
- 在上述基础下通过缓存等机制尽可能提高访存速度

### 重要机制&技术

![mmu](./mmu.png)

#### Paging

Paging是一种内存管理技术，它允许我们假设内存是连续的，即使实际上它是在物理内存的不连续区域上分配的。这通过将虚拟内存空间和物理内存空间划分为固定大小的块（页）来实现。每一个页都有一个独立的地址。MMU用于映射虚拟页到物理页。

#### TLB

转译后援缓冲（Translation Lookaside Buffer，TLB）是一种硬件实现，用于加速虚拟地址到物理地址的转换过程。每当进行地址转换时，MMU首先会查看TLB是否有相应的条目。如果有，就直接使用TLB中的信息进行转换，这被称为TLB命中。如果没有，那么就需要通过页表进行查找，并将结果缓存在TLB中，这被称为TLB缺失。

### 可行性分析

#### 硬件可行性

树莓派的CPU架构（ARM Cortex-A72）具有支持MMU的硬件，这意味着我们可以在FreeRTOS中实现MMU支持。

#### 软件可行性


这个mmu.c是什么？？ https://github.com/jameswalmsley/FreeRTOS/blob/master/FreeRTOS/Demo/CORTEX_A5_SAMA5D3x_Xplained_IAR/AtmelFiles/libchip_sama5d3x/source/mmu.c

调查一下arm的手册


根据之前组的[调查结果](https://github.com/OSH-2023/imagination/blob/main/docs/%E7%BB%93%E9%A2%98%E6%8A%A5%E5%91%8A/%E7%BB%93%E9%A2%98%E6%8A%A5%E5%91%8A.md)，我很不乐观

参考：

MMU Lecture slides(部分内容可以从这个借鉴)

-  https://cseweb.ucsd.edu/classes/su09/cse120/lectures/Lecture7.pdf
-  https://cseweb.ucsd.edu/classes/su09/cse120/lectures/Lecture8.pdf

Paging in MMU:
-  "Memory management unit" from Wikipedia. [Link](https://en.wikipedia.org/wiki/Memory_management_unit)
-  "CS 537 Lecture Notes Part 7 Paging" from University of Wisconsin-Madison. [Link](https://pages.cs.wisc.edu/~solomon/cs537-old/last/paging.html)

TLB in MMU:
- "Translation lookaside buffer" from Wikipedia. [Link](https://en.wikipedia.org/wiki/Translation_lookaside_buffer)
- "ARM Cortex-A Series Programmer's Guide for ARMv8-A" from ARM Developer. [Link](https://developer.arm.com/documentation/den0024/latest/The-Memory-Management-Unit/The-Translation-Lookaside-Buffer)
<!-- - "ARMv8-A Memory Management: MMU and TLB Explained" from LinkedIn. [Link](https://www.linkedin.com/advice/3/how-do-you-leverage-memory-management-unit-mmu) -->

MMU support in Raspberry Pi CPU architecture:
- "MMU on Raspberry Pi" from snaums.de. [Link](https://www.snaums.de/informatik/mmu-on-raspberry-pi.html)
- "A Raspberry Pi Operating System for Exploring Advanced..." from University of Maine. [Link](https://web.eece.maine.edu/~vweaver/projects/vmwos/2018_memsys_os.pdf)

MMU support in FreeRTOS:
- "FreeRTOS MMU support - Kernel" from FreeRTOS Forums. [Link](https://forums.freertos.org/t/freertos-mmu-support/10509)
- "Benefits of Using the Memory Protection Unit" from FreeRTOS. [Link](https://www.freertos.org/2021/02/benefits-of-using-the-memory-protection-unit.html)
- "mmu.c" from GitHub repos by James Walmsley. [Link](https://github.com/jameswalmsley/FreeRTOS/blob/master/FreeRTOS/Demo/CORTEX_A5_SAMA5D3x_Xplained_IAR/AtmelFiles/libchip_sama5d3x/source/mmu.c)
