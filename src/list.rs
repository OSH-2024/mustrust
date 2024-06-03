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
    fn insert(&mut self, item_link: WeakItemLink) {
        println!("in");
        let value_of_insertion = get_weak_item_value(&item_link);
        let item_to_insert = if value_of_insertion == portMAX_DELAY {
            get_list_item_prev(&Arc::downgrade(&self.list_end))
        } else {
            let mut iterator = Arc::downgrade(&self.list_end);
            loop {
                /* There is nothing to do here, just iterating to the wanted
                insertion position. */
                let next = get_list_item_next(&iterator);
                if get_weak_item_value(&next) > value_of_insertion {
                    break iterator;
                }
                iterator = next;
            }
        };

        let prev = Weak::clone(&item_to_insert);
        let next = get_list_item_next(&item_to_insert);

        set_list_item_next(&item_link, Weak::clone(&next));
        set_list_item_prev(&item_link, Weak::clone(&prev));
        set_list_item_next(&prev, Weak::clone(&item_link));
        set_list_item_prev(&next, Weak::clone(&item_link));

        self.number_of_items += 1;
    }
}