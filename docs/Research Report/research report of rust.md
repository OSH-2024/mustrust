# Rust调研

## Rust是什么

Rust 是一门系统级别的编程语言，它旨在解决C/C++等语言在系统级编程中常见的安全性和并发性等问题。

传统的系统级编程语言如C/C++在处理内存安全性和并发性时存在许多挑战，如空指针、数据竞争等问题。而Rust的设计目标之一是通过强大的类型系统和所有权机制解决这些问题，提高代码的安全性和可靠性。

Rust语言具有以下特性：

1. **内存安全性**：Rust通过所有权系统、借用检查器和生命周期管理等机制，确保在编译时避免内存安全问题，如空指针、野指针、数据竞争等。这对于系统级编程至关重要，因为这些问题可能导致严重的系统崩溃或安全漏洞。
2. **并发性**：Rust内置了轻量级的线程模型和消息传递机制，使并发编程更加容易和安全。它的`async`/`await`语法和`Future`类型使异步编程变得简单而高效。
3. **性能**：Rust的性能与C/C++相当，甚至在某些情况下更好。它通过零成本抽象和内联优化等技术实现高性能。
4. **模式匹配**：Rust拥有强大的模式匹配功能，可用于处理复杂的数据结构和状态转换，使代码更加清晰和可维护。
5. **生态系统**：Rust拥有活跃的社区和丰富的库，可以轻松地与其他语言进行集成，并提供各种功能强大的工具和框架，如Cargo构建系统、Tokio异步运行时等。
6. **WASM支持:** Rust可以被编译为WebAssembly(WASM),使其代码可以在浏览器中运行,为操作系统带来了新的应用场景[5]。

Rust语言适用于以下几个主要群体：

**1. 开发者团队**

Rust 被证明是可用于大型的、拥有不同层次系统编程知识的开发者团队间协作的高效工具。底层代码中容易出现种种隐晦的 bug，在其他编程语言中，只能通过大量的测试和经验丰富的开发者细心的代码评审来捕获它们。在 Rust 中，编译器充当了守门员的角色，它拒绝编译存在这些难以捕获的 bug 的代码，这其中包括并发 bug。通过与编译器合作，团队将更多的时间聚焦在程序逻辑上，而不是追踪 bug。

Rust 也为系统编程世界带来了现代化的开发工具：

- Cargo，内置的依赖管理器和构建工具，它能轻松增加、编译和管理依赖，并使其在 Rust 生态系统中保持一致。
- Rustfmt 确保开发者遵循一致的代码风格。
- Rust Language Server 为集成开发环境（IDE）提供了强大的代码补全和内联错误信息功能。

通过使用 Rust 生态系统中的这些和其他工具，开发者可以在编写系统层面代码时保持高生产力。

**2. 学生**

Rust 适用于学生和对学习系统概念感兴趣的其他人。通过 Rust，很多人已经了解了操作系统开发等主题。社区也非常欢迎并乐于解答学生们的问题。

**3. 公司**

数以百计的公司，无论规模大小，都在生产中使用 Rust 来完成各种任务。这些任务包括命令行工具、web 服务、DevOps 工具、嵌入式设备、音视频分析与转码、加密货币（cryptocurrencies）、生物信息学（bioinformatics）、搜索引擎、物联网（internet of things, IOT）程序、机器学习，甚至还包括 Firefox 浏览器的大部分内容。

**4. 重视速度和稳定性的开发者**

Rust 适用于追求编程语言的速度与稳定性的开发者。所谓速度，是指你用 Rust 开发出的程序运行速度，以及 Rust 提供的程序开发速度。Rust 的编译器检查确保了增加功能和重构代码时的稳定性。这与缺少这些检查的语言形成鲜明对比，开发者通常害怕修改那些脆弱的遗留代码。力求零开销抽象（zero-cost abstractions），把高级的特性编译成底层的代码，这样写起来很快，运行起来也很快，Rust 致力于使安全的代码也同样快速。

## 为什么要选择Rust

### Rust相比于其他语言的优点

众所周知，众多操作系统内核都是由 C 语言编写而成的，但是由于设计原因，C 语言有灵活高效的指针操作，但是这些使得它的安全性不能保证，主要体现在：

- 空指针引用（NULL Dereference）
- 释放内存后再使用（Use After Free）
- 返回悬空指针（Dangling Pointers）
- 超出访问权限（Out Of Bounds Access）

声名狼藉的程序分段错误（Segmentation Fault）是 C 语言的常见问题，而通常 `NULL dereferences` 是第一大诱因。如果开发者忘记了检查所返回的指针是否正确性，就可能会导致空指针引用。Rust 处理这类指针错误的方式非常极端，在“安全”代码中粗暴简单地禁用所有裸指针。此外在“安全”代码中，Rust 还取消了空值。像 C++ 一样，Rust 也使用资源获取即初始化（Resource Acquisition Is Initializa-tion）的方式，这意味着每个变量在超出范围后都一定会被释放，因此在“安全的”Rust代码中，永远不必担心释放内存的事情。但 Rust 不满足于此，它更进一步，直接禁止用户访问被释放的内存。这一点通过 Ownership 规则实现，在 Rust 中，变量有一个所有权（Ownership）属性，owner 有权随意调用所属的数据，也可以在有限的 lifetime 内借出数据（Borrowing）。此外，数据只能有一个 owner，这样一来，通过 RAII 规则，owner 的范围指定了何时释放数据。最后，ownership 还可以被“转移”，当开发者将ownership 分配给另一个不同的变量时，ownership 就会转移。

此外，C语言中向 stack-bound 变量返回指针很糟糕，返回的指针会指向未定义内存。虽然这类错误多见于新手，一旦习惯堆栈规则和调用惯例，就很难出现这类错误了。事实证明，Rust 的 lifetime check 不仅适用于本地定义变量，也适用于返回值。与C 语言不同，在返回 reference 时，Rust 的编译器会确保相关内容可有效调用，也就是说，编译器会核实返回的 reference 有效。即 Rust 的 reference 总是指向有效内存。

还有一个常见问题就是在访问时，访问了没有权限的内存，多半情况就是所访问的数组，其索引超出范围。这种情况也出现在读写操作中，访问超限内存会导致可执行文件出现严重的漏洞，这些漏洞可能会给黑客操作你的代码大开方便之门。著名的就是`Heartbleed bug`问题。

除了高安全性，Rust的编译器在生成高效的机器码方面做了大量优化工作,使得Rust程序几乎可以达到C程序的运行效率,因此完全可以用于撰写性能敏感的系统软件。

综上，`Rust`是一门兼顾了内存安全性与运行效率的编程语言，十分适合于系统级编程。许多知名的公司与组织如微软，谷歌，亚马逊等都在其项目中使用了Rust并尝试使用Rust改写以前的项目。

### 用Rust编写嵌入式操作系统的优势

1. 运行时库
   编程语言的运行时库，通常理解为，其编译出的可执行程序在运行时必须依赖的非操作系统本身的动态库。例如 C 程序必须依赖 msvcrt 或 glibc，Java 程序必须依赖 JRE，VB 程序必须依赖 msvbvm，易语言程序必须依赖 krnln.fne/fnr，等等。由于 C 运行时库往往跟操作系统紧密集成（尤其是类 Unix 系统），可以认为 C 运行时库是操作系统的一部分，进而认为 C 没有运行时库（有争议）。如果认同这一点，那么，经过静态编译生成的 Rust 程序，运行时仅依赖 C 运行时库，也就可以认为没有运行时库了。即使不认同这一点，等以后 Rust 支持了静态链
   接 MUSL 库（同时抛弃掉 glibc），依然能够做到没有运行时库。当然，动态编译的 Rust 程序中运行时还是必须依赖标准库 libstd-.so 等动态库的，这是给予程序员的额外可选项。没有运行时库的优势在于，运行时库本身也具有平台依赖性或运行时依赖性，没有运行时库，则程序的所有代码都是程序员可控的。

2. 运行时损耗
   程序的运行时损耗，是指程序在运行过程中所必须付出的额外的代价。例如 Java的虚拟机、C# 的垃圾回收器、脚本语言的解释器等等，这些子系统本身在运行时都会消耗数量可观的内存和 CPU，影响程序和系统的运行性能。而 Rust 没有虚拟机、垃圾回收器和解释器，所以没有这类运行时损耗。此外，内存管理、栈管理、调用操作系统 API 和 C 库等各种情况下，都有可能产生额外的运行时损耗。Rust 运行时需要每个函数执行 morestack 检查栈溢出（morestack 已被取消），为了内存安全这是“必需的”检查，而以 C 语言的思路去看可能就是“额外的”损
   耗，无论如何这项运行时损耗很小。Unwinding 仅发生在 panic 之后，不视为运行时损耗。Rust 采用 jemalloc 管理内存（也可禁用），不仅没有运行时损耗，反而带来运行效率的明显提升。Rust 的 Rc 类型以引用计数管理对象内存，Arc 类型以 Atomic 引用计数管理对象内存，这是较小的运行时损耗。但如果程序员不主动使用 Rc/Arc 类型，则无需为此付出额外的代价。Go 语言的协程调度器，当然也有运行时损耗，但这在某种程度上是程序实现自身功能的必要，算不上“额外的”代价，如果不需要此功能则损耗很小，故本文作者不视其为运行时损耗。而其通过 channel 共享内存、管理逐步连续增长的栈、调用 C 库和系统 API，则被视为运行时损耗，因为这些都是“非必要的”损耗，而且损耗还不小。那 Java 的 JIT 编译器在运行时把字节码编译为机器码，算不算运行时损耗呢？
   损耗肯定是有的，但仅在特定条件下触发，且其带来的收益可能远大于损耗，是提升运行性能的必要步骤，故不认为它引入了“额外的”代价，不视其为运行时损耗。而 Java 的虚拟机和垃圾收集器，显然是突出的运行时损耗。

3. 核心库
   Rust 核心库，可以理解为是经过大幅精简的标准库，它被应用在标准库不能覆盖
   到的某些少数特定领域，如嵌入式开发。核心库不依赖任何操作系统，也不提供文件 / 网络 / 多线程 / 内存申请释放相关的任何功能，因而可移植性更好、应用范围更广。在代码开头写上 # ! [no_std] 就代表放弃标准库，而使用核心库。核心库里面有：基础的接口性数据类型（参见上文，下同）、基础类型操作接口、常用的功能性数据类型、常用的宏定义、底层操作接口等，而且跟标准库 API 几乎是完全一致的；再配合 alloc 库（或自己定制的 alloc 库）又有了内存申请释放功能；再加上 collections 库，String/Vec/HashMap 等也有了。事实上从内部实现来说，标准库里的某些功能正是来源于核心库（以及 alloc/collections 等）。

4. 内存安全
   内存不安全的后果十分严重，“心脏出血”漏洞 (Heartbleed) 重创全球 IT 行业。其源于 OpenSSL【越界访问内存】。OS/GLIBC/JAVA/浏览器等频繁爆出重大安全漏洞，多数都与错误使用内存有关。传统 C/C++ 语言放弃解决内存安全问题，程序员因疏忽或犯错很容易制造内存安全漏洞。使用 GC 能基本保证内存安全，但牺牲了运行时性能。Rust 针对内存安全做了严格的限制以获得高安全性。

   可以安全地读写内存：
   • 在限定时间和空间范围内读写内存
   • 防止被他人意外修改或释放
   • 避免访问空指针和野指针
   也可以安全地释放内存
   • 在恰当的时机释放
   • 确保释放，不遗漏
   • 仅释放一次

   而 C 语言中可能产生指针越界，野指针，NULL 指针解引用，并发读写导致数据竞争，缓冲区溢出，段错误等各种危险操作。

总体来说，Rust语言具有很强的控制性和很高的安全性，且运行效率高，无 GC无 VM。

## 如何使用Rust进行改写

用Rust改写FreeRTOS主要可分为以下几部分工作：

1. **下载FreeRTOS源码并阅读理解FreeRTOS的结构和功能**：
   - 详细了解FreeRTOS的内部结构、核心功能和API。
   - 熟悉FreeRTOS中常用的任务管理、调度器、信号量、消息队列等机制。
2. **学习Rust语言**：
   - 对Rust语言的基本语法、所有权系统、借用规则、并发模型等特性有清晰的理解。
   - 熟悉Rust中常用的标准库和相关工具，如`std`库、Cargo构建系统等。
3. **设计Rust版本的FreeRTOS接口**：
   - 根据FreeRTOS的功能和API设计相应的Rust接口，保持与原有C代码的兼容性。
   - 使用Rust的类型系统和模块化设计，提供更安全、可靠的接口。
4. **逐步转换和实现**：
   - 从简单的功能或模块开始，逐步将原有的FreeRTOS C代码转换为Rust代码实现。
   - 注意处理内存管理、并发访问、错误处理等方面的问题，利用Rust的安全性和并发性特性优化代码。
5. **测试和验证**：
   - 编写测试用例，验证Rust版本的FreeRTOS在功能和性能上与原有版本保持一致。
   - 进行单元测试、集成测试和性能测试，确保代码的正确性和稳定性。
6. **优化和改进**：
   - 结合Rust语言的特性对代码进行优化和改进，如使用`async`/`await`实现异步任务、利用泛型提高代码复用性等。
   - 注意处理内存占用、性能瓶颈等问题，确保Rust版本的FreeRTOS在实际应用中表现良好。

## 将会面临的挑战

用Rust改写FreeRTOS并不是一项简单的工作，预计改写过程将会面临以下挑战：

1. **学习成本**：由于团队成员都未曾接触过Rust语言，对Rust不熟悉，加之Rust语言引入了所有权等新的概念，其学习曲线也较为陡峭，故需要较长时间学习和适应Rust的语法、特性和开发工具。
2. **内存管理**：Rust的所有权系统和借用规则在内存管理方面非常严格，这可能与FreeRTOS的内存管理机制产生冲突。需要仔细处理内存分配和释放，确保在Rust中也能有效地管理内存，并避免内存泄漏和野指针等问题。
3. **并发性**：Rust的并发模型和线程安全性要求较高，与FreeRTOS的并发机制可能有一定差异。需要设计和实现并发任务调度、同步互斥等功能，保证在Rust中的并发操作与FreeRTOS保持一致并且安全可靠。
4. **C语言兼容性**：由于FreeRTOS是用C语言编写的，而Rust与C的互操作性需要谨慎处理。需要设计良好的接口和数据结构，确保Rust版本的FreeRTOS能够与现有C代码进行交互，并保持功能的完整性和一致性。
5. **性能优化**：Rust的性能与C语言相比可能会有一定差距，特别是在系统级编程领域。需要进行性能分析和优化，利用Rust的特性和工具提高代码的执行效率和资源利用率。
6. **工具和库的支持**：Rust相比C语言在某些领域的工具和库可能较少，需要寻找合适的工具和库来支持FreeRTOS的功能和扩展。
7. **社区支持**：Rust相比其他主流语言如C/C++的社区规模可能较小，需要依靠有限的社区资源获取支持和解决问题。

好在已经有前人做过这个项目可以为我们提供参考，此外Rust拥有非常全面和易于理解的[官方文档]([Rust 程序设计语言 - Rust 程序设计语言 中文版 (rustwiki.org)](https://rustwiki.org/zh-CN/book/title-page.html))和教程，包括"The Rust Programming Language"（通常称为Rust Book）、Rust by Example等可以帮助我们较快入门和掌握基本概念。
