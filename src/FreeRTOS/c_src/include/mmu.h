#ifndef MMU_H
#define MMU_H

#include <stdlib.h>

typedef struct line {
	char valid;
	char ref;
	char dirty;
	int page_number;
	int frame_number;
} line;

typedef struct entry {
	char valid;
	char dirty;
	int frame_number;
	int disk_address;
} entry;

typedef struct TCB {
	char *name;
	entry* page_table;
} TCB;

typedef struct NODE {
	int frame_number;
	struct NODE* next;
	int page_number;
	TCB *task_belonging;
} NODE, *LINKNODE;

void pInitList(LINKNODE*);//链表初始化
int pInsertElem(LINKNODE*, LINKNODE, int); //插入一个节点，节点位置从1开始，头节点位置为0
int pMovetoFirst(LINKNODE*, int); //从将元素e从当前位置移动到第一个位置，失败
LINKNODE GetEndNode(LINKNODE); //得到末尾节点的指针
int pTraverseList(LINKNODE);

#define useTLB	1
#define ReplacementStrategy 0	//0-LRU，1-FIFO

#define times 8
#define page_size (128 * times)  //页大小
#define memory_size (1024 * times)   //内存大小
#define memory_frame_size (memory_size / page_size)   //内存页总数
#define disk_size (4096 * times) //外存大小
#define virtual_space (2048 * times) //虚拟地址空间
#define page_table_size (virtual_space / page_size) //页表大小
#define TLB_size (4 * times) //TLB大小
#define time_TLB_access 1   //访问TLB时间
#define time_memory_access 100  //访问memory时间  
#define	time_cach_access 100	//访问cach时间
#define time_disk_access 1000000    //访问硬盘时间
#define start_address 0 //程序数据存放起始地址

extern int memory[memory_size];
extern int disk[disk_size];
extern line TLB[TLB_size];
extern TCB *currentTCB;
extern long int TLB_hit;
extern long int TLB_miss;
extern long int memory_hit;
extern long int memory_miss;
extern long int time_cost;

enum memory_operation {
	read = 0,
	write = 1
};

void page_table_init(TCB *tcb);
void FIFO_list_init(LINKNODE*);
int address_map(int virtual_address, enum memory_operation operation);
int read_to_memory(int memory_frame, int disk_frame);
int write_to_disk(int memory_frame, int disk_start_address);
void pageFault(entry * faultPage, int page_number);
void LRU_list_init(LINKNODE*);
int read_memory(int virtual_address);
void write_memory(int virtual_address, int data);
int TLB_search(int virtual_address, enum memory_operation operation);
int TLB_update(int page_number, int frame_number);
void write_back();

void initialize_tcb();
void uninitialize_tcb();
void initialize_list();
void initialize_stat();

#endif