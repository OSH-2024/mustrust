# mustrust小组期末汇报
- [mustrust小组期末汇报](#mustrust小组期末汇报)
  - [引言](#引言)
    - [项目背景](#项目背景)
      - [FreeRTOS](#freertos)
      - [Rust](#rust)
    - [小组分工](#小组分工)
    - [项目成果概述](#项目成果概述)
      - [实现的功能](#实现的功能)
      - [同类项目对比](#同类项目对比)
  - [Rust改写](#rust改写)
    - [原始FreeRTOS分析](#原始freertos分析)
    - [基本方法与原则](#基本方法与原则)
    - [遇到的困难与解决方案](#遇到的困难与解决方案)
      - [Rust与C的相互调用](#rust与c的相互调用)
      - [所有权机制](#所有权机制)
      - [const常量重复定义](#const常量重复定义)
      - [具有函数功能的宏被bindgen忽略](#具有函数功能的宏被bindgen忽略)
      - [适配上板——no-std环境](#适配上板no-std环境)
    - [关键模块](#关键模块)
      - [list模块](#list模块)
      - [task模块](#task模块)
  - [MMU部分](#mmu部分)
    - [关键函数](#关键函数)
    - [模拟结果](#模拟结果)
  - [交叉编译与上板](#交叉编译与上板)
    - [bindgen方案](#bindgen方案)
    - [链接](#链接)
    - [QEMU模拟与调试](#qemu模拟与调试)
    - [整体测试结果](#整体测试结果)
  - [总结](#总结)
    - [项目成果回顾](#项目成果回顾)
    - [未来工作展望](#未来工作展望)


## 引言

### 项目背景

#### FreeRTOS

FreeRTOS是一个广泛使用的实时操作系统(RTOS)，专为嵌入式系统设计。它由Richard Barry于2003年首次发布并持续维护。FreeRTOS以其简洁、高效、可裁剪的特点,在全球范围内获得了广泛的认可和应用，其优点包括但不限于以下几点：

* **流行性**：FreeRTOS是最主流的嵌入式操作系统之一，在全球范围内被广泛采用，拥有庞大的社区支持和活跃的生态系统。

* **功能丰富**：FreeRTOS提供了丰富的功能，如任务管理、内存管理、同步机制等，同时还支持文件系统，满足了嵌入式系统的各种需求。

* **性能优异**：FreeRTOS实时性能出色，能够满足嵌入式系统的实时性需求。

* **源代码质量**：FreeRTOS代码精简、可读性强，易于移植到各种嵌入式平台，对开发者友好。

* **版权“友好”**：FreeRTOS的许可证商业友好，允许商业应用和修改，没有版权的后顾之忧。

#### Rust

Rust 是一门系统级别的编程语言，它旨在解决C/C++等语言在系统级编程中常见的安全性和并发性等问题。

传统的系统级编程语言如C/C++在处理内存安全性和并发性时存在许多挑战，如空指针、数据竞争等问题。而Rust的设计目标之一是通过强大的类型系统和所有权机制解决这些问题，提高代码的安全性和可靠性。

Rust的优点包括但不限于以下几点：

* **内存安全性**：Rust通过所有权系统、借用检查器和生命周期管理等机制，确保在编译时避免内存安全问题，如空指针、野指针、数据竞争等

* **并发性**：Rust内置了轻量级的线程模型和消息传递机制，使并发编程更加容易和安全	

* **性能**：Rust的性能与C/C++相当，甚至在某些情况下更好。它通过零成本抽象和内联优化等技术实现高性能。

* **模式匹配**：Rust拥有强大的模式匹配功能，可用于处理复杂的数据结构和状态转换，使代码更加清晰和可维护。

* **生态系统**：Rust拥有活跃的社区和丰富的库，可以轻松地与其他语言进行集成，并提供各种功能强大的工具和框架，如Cargo构建系统等。

**Rust语言特性对我们项目开发的启发和要求**：

* 所有权机制

  Rust确保了内存安全，防止了缓冲区溢出、越界访问等常见的内存错误 => 要求我们在编写代码时必须仔细思考内存的使用和分配，防止代码中出现未预料到的更改从而影响代码的正确性和安全性。

* 生命周期管理与借用规则

  Rust提供了丰富的生命周期管理功能和借用规则 => 要求我们在编写代码时必须考虑数据的有效期和访问权限，提升代码的安全性，增强软件的可维护性和可读性。

* 类型安全与借用检测器

  Rust的借用检查器保证了代码的类型安全，能够在编译时期就发现潜在的类型错误 => 要求我们在编写代码时必须严格遵守类型规则，确保数据类型的正确匹配，以避免程序在运行时期的类型错误。

### 项目成果概述

#### 实现的功能

* Rust版本的FreeRTOS系统顺利在QEMU上运行
* MMU功能顺利实现

#### 同类项目对比

![同类项目对比](./img/同类项目对比.png)

## Rust改写

### 原始FreeRTOS分析



### 基本方法与原则

**基本方法**：将源码的函数进行拆分为各种简单的函数或方法，然后整合

**改写原则**：

* 保留源码中的所有函数，函数的命名相应的改为符合Rust命名规范的版本，如`vInitialiseList`改为`initialize_list`
* 渐进式“换血”：重写核心模块 -> 封装原有C源码 -> 测试验证 -> 逐步替换 -> 性能优化

### 遇到的困难与解决方案

#### Rust与C的相互调用



#### 所有权机制

**问题**：所有权问题在链表中尤为突出，因为链表中的链表项可能会在多个地方被使用，它们在使用完后会被释放，而这会导致链表项的生命周期提前结束。

**解决方案**：使用数据结构Arc将数据包裹，它能够统计程序的不同地方对某个变量的引用，并且进行计数。只要存在这样的引用，程序就不会自动释放这个变量，从而确保了变量的有效性。

**但是**，使用Arc包裹数据又会产生新的问题——**循环引用**，例如A引用B，B引用A，那么A，B就永远无法被释放！

**最终解决方案**：再引入弱引用Weak，它不会增加引用数。同时，利用Rust的upgrade和downgrade函数可以实现强引用Arc和弱引用Weak之间的转换，合理增减变量的引用数。

#### const常量重复定义

**问题**：编译时报错头文件中的const常量重复定义，如下图。

![重复定义](./img/重复定义.png)

**解决方案**：将重复定义的const常量改成宏定义的形式，如下图：

![const常量修改为宏](./img/const常量修改为宏_2.png)

#### 具有函数功能的宏被bindgen忽略

**问题**：在使用bindgen时，头文件中的一部分具有函数功能的宏会被忽略。

**解决方案**：

**方案一**：专门写一个函数来扫描所有头文件，将具有函数功能的宏以macro的形式追加到bindgen末尾。

**方案二**：手动将所有具有函数功能的宏修改为具有相同名称和功能的函数。

由于这样的宏并不是很多，所以我们选择了**方案二**，下面就是一个修改宏的例子：

![将函数功能的宏修改为函数](./img/将函数功能的宏修改为函数.png)

#### 适配上板——no-std环境

**问题**：上板时为`no-std`环境，这意味着不支持`std`库，而我们的代码中大量使用了`std`库的功能。

**解决方案**：使用`no-st`d环境支持的库如`core`，`alloc`等来替代`std`：

`core`：核心库，提供Rust语言的基础设施，如基础数据类型、操作符、宏等。

`alloc`：提供动态内存分配的支持。

`no_std_async`：专为no-std环境设计的异步库，旨在提供不依赖标准库的情况下使用异步编程的能力。

在代码的具体实现中，我们做了以下替换：

```rust
std::sync => alloc::sync
std::fmt => core::fmt
std::cell::UnsafeCell => core::cell::UnsafeCell
std::collections::VecDeque => alloc::collections::VecDeque
std::boxed => alloc::boxed
std::mem => core::mem
std::sync::RwLock => synctools::rwlock
```

此外，在实现某些功能时，还引入了诸如`core::marker::PhantomData`等no-std环境的库中的内容。

### 关键模块

#### list模块

##### list介绍

在`FreeRTOS`中，List是一个双向链表，其主要任务是辅助任务调度。

在`list.h`中，每个链表`xList`由链表项`xList_Item`，链表项个数，链表的结束标记和用于进行数据完整性检查的两个条件编译字段组成，其结构体定义如下：

```c
typedef struct xLIST
{
	listFIRST_LIST_INTEGRITY_CHECK_VALUE
	volatile UBaseType_t uxNumberOfItems;
	ListItem_t * configLIST_VOLATILE pxIndex
	MiniListItem_t xListEnd;
	listSECOND_LIST_INTEGRITY_CHECK_VALUE
} List_t;
```

链表项`xListItem`由节点值，指向前后结点的指针，指向拥有当前链表节点的对象的指针（通常，这个对象是一个任务控制块`TCB`），指向包含当前链表节点的链表的指针和用于进行数据完整性检查的两个条件编译字段组成，其结构体定义如下：

```c
struct xLIST_ITEM
{
	listFIRST_LIST_ITEM_INTEGRITY_CHECK_VALUE
	configLIST_VOLATILE TickType_t xItemValue;
	struct xLIST_ITEM * configLIST_VOLATILE pxNext;
	struct xLIST_ITEM * configLIST_VOLATILE pxPrevious;
	void * pvOwner;
	void * configLIST_VOLATILE pvContainer;
	listSECOND_LIST_ITEM_INTEGRITY_CHECK_VALUE
};
```

##### Rust改写方式

前面已经提到过，Rust 语言的所有权机制，给实现 List 带来了问题——链表中的链表项可能会在多个地方被使用，它们在使用完后会被释放，而这会导致链表项的生命周期提前结束。一个解决方案是使用数据结构Arc将数据包裹，它能够统计程序的不同地方对某个变量的引用，并且进行计数。只要存在这样的引用，程序就不会自动释放这个变量，从而确保了变量的有效性。

但是使用Arc包裹数据又会产**循环引用**的问题，例如A引用B，B引用A，那么A，B就永远无法被释放！

最终的解决方案为再引入弱引用Weak，它不会增加引用数。同时，利用Rust的upgrade和downgrade函数可以实现强引用Arc和弱引用Weak之间的转换，合理增减变量的引用数。

Rust版本的链表项和链表的数据结构如下：

```rust
pub struct xLIST_ITEM
{
	xItemValue: TickType,	// 辅助值，用于帮助结点做顺序排列
	
	pxNext: WeakItemLink,	// 双向引用
	pxPrevious: WeakItemLink,	// 双向引用
    pvOwner: Weak<RwLock<TaskControlBlock>>,	// 指向拥有该结点的内核对象
    pvContainer: Weak<RwLock<List_t>>,	// 指向该节点所在的链表 双向引用
}

pub struct xLIST
{
    uxNumberOfItems: UBaseType,	// 链表节点计数器
	pxIndex: WeakItemLink,	// 链表节点索引指针			
    xListEnd: ItemLink,	// 链表最后一个节点 单向引用					
}
```

然后，我们采取将源码的函数进行拆分为各种简单的函数或方法，然后整合的方式来实现`list`模块的功能：

1. `xLIST_ITEM::default`：`xLIST_ITEM`结构体的默认构造函数，用于初始化一个链表项，其中`xItemValue`设置为`portMAX_DELAY`，表示这个项在排序时会被放在链表的最末尾。
2. `xLIST_ITEM::new`：创建一个新的链表项，并设置其`xItemValue`。
3. `xLIST_ITEM::set_value`：设置链表项的`xItemValue`。
4. `xLIST_ITEM::owner`：设置链表项的拥有者，通常是一个任务控制块（`TaskControlBlock`）。
5. `xLIST_ITEM::set_container`：设置链表项所属的容器，即它所在的链表。
6. `xLIST_ITEM::remove`：从链表中移除一个项。
7. `new_list_item`：创建一个新的链表项，并用`Arc<RwLock>`包装，以便在多线程环境中安全访问。
8. `xLIST::default`：初始化一个空的链表，其中包含一个特殊的链表末端项，用于标记链表的结束。
9. `xLIST::traverse`：遍历链表的函数，示例中未完全实现，通常用于访问链表中的每个项。
10. `xLIST::insert`：将一个项插入到链表中，插入位置根据项的`xItemValue`确定，以保持链表的排序。
11. `xLIST::insert_end`：在链表的末尾插入一个项。
12. `xLIST::remove_item`：从链表中移除一个项。
13. `xLIST::is_empty`：检查链表是否为空。
14. `xLIST::get_length`：获取链表的长度，即其中项的数量。
15. `xLIST::increment_index`：移动链表的内部索引指针，用于遍历链表。
16. `xLIST::get_owner_of_next_entry`：获取链表中下一个项的拥有者。
17. `xLIST::get_owner_of_head_entry`：获取链表头部项的拥有者。
18. `set_list_item_next`和`set_list_item_prev`：这两个函数用于设置链表项的`pxNext`和`pxPrevious`指针，分别指向链表中的下一个和上一个项。

完整代码见仓库中的[list.rs](https://github.com/OSH-2024/mustrust/blob/main/src/FreeRTOS/src/list.rs)

然后我们就可以使用这其中的函数或方法来很容易的实现原`list.c`的功能：

* `list_initialise(item: &mut ItemLink)`：直接调用相应的`xList::default()`方法即可。

* `list_initialiseItem(item: &mut ListItem_t)`：直接调用相应的`xList_Item::default()`方法即可。

* `list_insert(list: &ListLink, item_link: &ItemLink)`：首先通过`write`方法获得待插入节点的可变引用，然后调用`set_container`方法将这个节点与其所属的链表关联起来。这个关联对于之后如果需要从链表中快速移除该节点是非常有用的，因为它允许直接定位到节点所在的链表。然后通过`write`方法获得链表的可变引用，最后调用链表的`insert`方法将节点插入到链表中。注意插入时使用的是`Arc::downgrade(&item_link)`以将`item_link`的`Arc`（一个智能指针，用于提供线程安全的引用计数所有权）转换为一个`Weak`指针，避免循环引用。

* `list_insert_end(list: &ListLink, item_link: &ItemLink)`：与`list_insert`类似，不同的是最后调用的是链表的`insert_end`方法将节点插入到链表末尾。
* `list_remove(item_link: ItemLink) -> UBaseType`：首先通过`item_link.write()`调用获取到`item_link`的可变引用。然后调用链表的`remove`方法从链表中移除`item_link`指向的节点。同样需要注意的是，这里使用的是`Arc::downgrade(&item_link)`作为参数，原因同`list_insert`。最后，`remove`方法返回链表中剩余节点的数量，类型为`UBaseType`。

代码如下：

```rust
pub fn list_initialise(item: &mut ItemLink) {
    let ItemLink = xLIST::default();
}

pub fn list_initialiseItem(item: &mut ListItem_t) {
    let ListItem = xLIST_ITEM::default();
}

pub fn list_insert(list: &ListLink, item_link: &ItemLink) {
    item_link.write().set_container(&list);
    list.write().insert(Arc::downgrade(&item_link))
}

pub fn list_insert_end(list: &ListLink, item_link: &ItemLink) {
    item_link.write().set_container(&list);
    list.write().insert_end(Arc::downgrade(&item_link))
}

pub fn list_remove(item_link: ItemLink) -> UBaseType {
    item_link
        .write()
        .remove(Arc::downgrade(&item_link))
}
```



#### task模块



## MMU部分

### 关键函数



### 模拟结果



## 交叉编译与上板

### bindgen方案



### 链接



### QEMU模拟与调试



### 整体测试结果



## 总结

### 项目成果回顾



### 未来工作展望



