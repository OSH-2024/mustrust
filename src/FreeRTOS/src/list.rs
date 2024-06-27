use std::fmt;
use std::sync::{Arc, RwLock, Weak};

use crate::port::{portMAX_DELAY, TickType, UBaseType};
use crate::task_control::{TaskHandle, TCB};

impl fmt::Debug for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListItem with value: {}", self.item_value)
    }
}

pub struct ListItem {
    item_value: TickType,
    next: WeakItemLink,
    prev: WeakItemLink,
    owner: Weak<RwLock<TCB>>,
    container: Weak<RwLock<List>>,
}

pub type ItemLink = Arc<RwLock<ListItem>>;
pub type WeakItemLink = Weak<RwLock<ListItem>>;
pub type WeakListLink = Weak<RwLock<List>>;
pub type ListLink = Arc<RwLock<List>>;

impl Default for ListItem {
    fn default() -> Self {
        ListItem {
            item_value: portMAX_DELAY,
            next: Default::default(),
            owner: Default::default(),
            prev: Default::default(),
            container: Default::default(),
        }
    }
}

impl ListItem {
    pub fn item_value(mut self, item_value: TickType) -> Self {
        self.item_value = item_value;
        self
    }

    pub fn owner(mut self, owner: TaskHandle) -> Self {
        self.owner = owner.into();
        self
    }

    pub fn set_container(&mut self, container: &Arc<RwLock<List>>) {
        self.container = Arc::downgrade(container);
    }

    fn remove(&mut self, link: WeakItemLink) -> UBaseType {
        /* The list item knows which list it is in.  Obtain the list from the list
        item. */
        let list = self
            .container
            .upgrade()
            .unwrap_or_else(|| panic!("Container not set"));
        let ret_val = list.write().unwrap().remove_item(&self, link);
        set_list_item_next(&self.prev, Weak::clone(&self.next));
        set_list_item_prev(&self.next, Weak::clone(&self.prev));
        self.container = Weak::new();
        ret_val
    }
}

#[derive(Clone)]
pub struct List {
    number_of_items: UBaseType,
    /* Used to walk through the list.
     * Points to the last item returned by a call to listGET_OWNER_OF_NEXT_ENTRY (). */
    index: WeakItemLink,
    list_end: ItemLink,
}

impl Default for List {
    fn default() -> Self {
        /* The list structure contains a list item which is used to mark the
        end of the list.  To initialise the list the list end is inserted
        as the only list entry. */
        let list_end: ItemLink = Arc::new(RwLock::new(ListItem::default()));

        /* The list end next and previous pointers point to itself so we know
        when the list is empty. */
        list_end.write().unwrap().next = Arc::downgrade(&list_end);
        list_end.write().unwrap().prev = Arc::downgrade(&list_end);

        List {
            index: Arc::downgrade(&list_end),
            list_end: list_end,
            number_of_items: 0,
        }
    }
}

fn set_list_item_next(item: &WeakItemLink, next: WeakItemLink) {
    let owned_item = item
        .upgrade()
        .unwrap_or_else(|| panic!("List item is None"));
    (*owned_item.write().unwrap()).next = next;
}

fn set_list_item_prev(item: &WeakItemLink, prev: WeakItemLink) {
    let owned_item = item
        .upgrade()
        .unwrap_or_else(|| panic!("List item is None"));
    (*owned_item.write().unwrap()).prev = prev;
}

fn get_list_item_next(item: &WeakItemLink) -> WeakItemLink {
    let owned_item = item
        .upgrade()
        .unwrap_or_else(|| panic!("List item is None"));
    let next = Weak::clone(&(*owned_item.read().unwrap()).next);
    next
}

fn get_list_item_prev(item: &WeakItemLink) -> WeakItemLink {
    let owned_item = item
        .upgrade()
        .unwrap_or_else(|| panic!("List item is None"));
    let prev = Weak::clone(&(*owned_item.read().unwrap()).prev);
    prev
}


pub fn get_list_item_value(item: &ItemLink) -> TickType {
    item.read().unwrap().item_value
}

pub fn set_list_item_value(item: &ItemLink, item_value: TickType) {
    item.write().unwrap().item_value = item_value;
}

fn get_weak_item_value(item: &WeakItemLink) -> TickType {
    let owned_item = item
        .upgrade()
        .unwrap_or_else(|| panic!("List item is None"));
    let value = owned_item.read().unwrap().item_value;
    value
}

fn set_weak_item_value(item: &WeakItemLink, item_value: TickType) {
    let owned_item = item
        .upgrade()
        .unwrap_or_else(|| panic!("List item is None"));
    owned_item.write().unwrap().item_value = item_value;
}

pub fn get_list_item_container(item: &ItemLink) -> Option<ListLink> {
    //let owned_item = item.upgrade().unwrap_or_else(|| panic!("List item is None"));
    let container = Weak::clone(&item.read().unwrap().container);
    container.upgrade()
}

pub fn list_is_empty(list: &ListLink) -> bool {
    list.read().unwrap().is_empty()
}

pub fn current_list_length(list: &ListLink) -> UBaseType {
    list.read().unwrap().get_length()
}

pub fn get_list_item_owner(item_link: &ItemLink) -> TaskHandle {
    let owner = Weak::clone(&item_link.read().unwrap().owner);
    owner.into()
}

pub fn set_list_item_owner(item_link: &ItemLink, owner: TaskHandle) {
    item_link.write().unwrap().owner = owner.into()
}

pub fn get_owner_of_next_entry(list: &ListLink) -> TaskHandle {
    let task = list.write().unwrap().get_owner_of_next_entry();
    task.into()
}

pub fn get_owner_of_head_entry(list: &ListLink) -> TaskHandle {
    let task = list.read().unwrap().get_owner_of_head_entry();
    task.into()
}

pub fn is_contained_within(list: &ListLink, item_link: &ItemLink) -> bool {
    match get_list_item_container(&item_link) {
        Some(container) => Arc::ptr_eq(list, &container),
        None => false,
    }
}

pub fn list_insert(list: &ListLink, item_link: ItemLink) {
    /* Remember which list the item is in.  This allows fast removal of the
    item later. */
    item_link.write().unwrap().set_container(&list);
    println!("Set conatiner");
    list.write().unwrap().insert(Arc::downgrade(&item_link))
}

pub fn list_insert_end(list: &ListLink, item_link: ItemLink) {
    /* Insert a new list item into pxList, but rather than sort the list,
    makes the new list item the last item to be removed by a call to
    listGET_OWNER_OF_NEXT_ENTRY(). */

    /* Remember which list the item is in. */
    item_link.write().unwrap().set_container(&list);

    list.write().unwrap().insert_end(Arc::downgrade(&item_link))
}

pub fn list_remove(item_link: ItemLink) -> UBaseType {
    item_link
        .write()
        .unwrap()
        .remove(Arc::downgrade(&item_link))
}

impl List {
    // 定义一个函数 `insert`，它接受一个弱引用的链表项 `item_link` 作为参数
    fn insert(&mut self, item_link: WeakItemLink) {
        // 打印调试信息
        println!("in");
        // 获取待插入项的值
        let value_of_insertion = get_weak_item_value(&item_link);
        // 根据待插入项的值决定插入的位置
        let item_to_insert = if value_of_insertion == portMAX_DELAY {
            // 如果待插入项的值等于 `portMAX_DELAY`，则插入到链表的末尾
            get_list_item_prev(&Arc::downgrade(&self.list_end))
        } else {
            // 否则，从链表的末尾开始遍历链表
            let mut iterator = Arc::downgrade(&self.list_end);
            loop {
                // 获取当前项的下一个项
                let next = get_list_item_next(&iterator);
                // 如果下一个项的值大于待插入项的值，那么就在当前位置插入待插入项
                if get_weak_item_value(&next) > value_of_insertion {
                    break iterator;
                }
                // 否则，继续遍历链表
                iterator = next;
            }
        };

        // 获取待插入位置的前一个项和下一个项
        let prev = Weak::clone(&item_to_insert);
        let next = get_list_item_next(&item_to_insert);

        // 设置待插入项的前一个项和下一个项
        set_list_item_next(&item_link, Weak::clone(&next));
        set_list_item_prev(&item_link, Weak::clone(&prev));
        // 设置前一个项的下一个项和下一个项的前一个项为待插入项
        set_list_item_next(&prev, Weak::clone(&item_link));
        set_list_item_prev(&next, Weak::clone(&item_link));

        // 链表项的数量增加 1
        self.number_of_items += 1;
    }

    // 定义一个函数 `insert_end`，它接受一个弱引用的链表项 `item_link` 作为参数
    fn insert_end(&mut self, item_link: WeakItemLink) {
        // 获取当前链表索引项的前一个项
        let prev = get_list_item_prev(&self.index);
        // 克隆当前链表索引项的弱引用
        let next = Weak::clone(&self.index);
        // 设置 `item_link` 的下一个项为 `next`
        set_list_item_next(&item_link, Weak::clone(&next));
        // 设置 `item_link` 的前一个项为 `prev`
        set_list_item_prev(&item_link, Weak::clone(&prev));
        // 设置 `prev` 的下一个项为 `item_link`
        set_list_item_next(&prev, Weak::clone(&item_link));
        // 设置 `next` 的前一个项为 `item_link`
        set_list_item_prev(&next, Weak::clone(&item_link));
        // 链表项的数量增加 1
        self.number_of_items += 1;
    }

    fn remove_item(&mut self, item: &ListItem, link: WeakItemLink) -> UBaseType {
        // TODO: Find a more effiecient
        if Weak::ptr_eq(&link, &self.index) {
            self.index = Weak::clone(&item.prev);
        }
        self.number_of_items -= 1;
        self.number_of_items
    }

    fn is_empty(&self) -> bool {
        self.number_of_items == 0
    }

    fn get_length(&self) -> UBaseType {
        self.number_of_items
    }

    fn increment_index(&mut self) {
        self.index = get_list_item_next(&self.index);
        if Weak::ptr_eq(&self.index, &Arc::downgrade(&self.list_end)) {
            self.index = get_list_item_next(&self.index);
        }
    }

    fn get_owner_of_next_entry(&mut self) -> Weak<RwLock<TCB>> {
        self.increment_index();
        let owned_index = self
            .index
            .upgrade()
            .unwrap_or_else(|| panic!("List item is None"));
        let owner = Weak::clone(&owned_index.read().unwrap().owner);
        owner
    }

    fn get_owner_of_head_entry(&self) -> Weak<RwLock<TCB>> {
        let list_end = get_list_item_next(&Arc::downgrade(&self.list_end));
        let owned_index = list_end
            .upgrade()
            .unwrap_or_else(|| panic!("List item is None"));
        let owner = Weak::clone(&owned_index.read().unwrap().owner);
        owner
    }
}

pub fn vListInitialise(item: &mut ItemLink) {
    let ItemLink = List::default();
}

pub fn vListInitialiseItem(item: &mut ListItem) {
    let ListItem = ListItem::default();
}

// 将节点按照升序排列插入到链表
pub fn vListInsert(list: &ListLink, item_link: &ItemLink) {
    item_link.write().unwrap().set_container(&list);
    println!("Set conatiner");
    list.write().unwrap().insert(Arc::downgrade(&item_link))
}


// list_insert_end
pub fn vListInsertEnd(list: &ListLink, item_link: &ItemLink) {
    item_link.write().unwrap().set_container(&list);
    list.write().unwrap().insert_end(Arc::downgrade(&item_link))
}

// list_remove
pub fn uxListRemove(item_link: ItemLink) -> UBaseType_t {
    item_link
        .write()
        .unwrap()
        .remove(Arc::downgrade(&item_link))
}