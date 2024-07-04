#include "mmu.h"
#include "projdefs.h"
#include "portmacro.h"

LINKNODE LRU_list;
LINKNODE FIFO_list;

void pInitList(LINKNODE *pHead)
{
	*pHead = (LINKNODE)pvPortMalloc(sizeof(NODE));
	(*pHead)->next = 0;
};


int pInsertElem(LINKNODE *pHead, LINKNODE s, int posi)
{
	LINKNODE p;
	int counter = 0;
	if (*pHead == 0)
	{
		return -1;
	}
	p = *pHead;
	while (p != 0)
	{
		if (counter == posi - 1)
		{
			s->next = p->next;
			p->next = s;
			return 1;
		}
		p = p->next;
		counter++;
	}
	return -1;
};


int pMovetoFirst(LINKNODE *pHead, int e)
{
	LINKNODE p = *pHead;
	LINKNODE q;
	if (*pHead == 0) return -1;
	while ((p != 0) && (p->next->frame_number != e)) p = p->next;
	if (p == 0) return -1;
	q = p->next;
	p->next = q->next;
	q->next = (*pHead)->next;
	(*pHead)->next = q;
	return 1;
}


LINKNODE GetEndNode(LINKNODE pHead)
{
	LINKNODE p = pHead;
	if (pHead == 0) return 0;
	while (p->next != 0) p = p->next;
	return p;
}

int memory[memory_size];
int disk[disk_size];
TCB *currentTCB;
line TLB[TLB_size];

int replacement_number_FIFO;
long int time_cost;
long int TLB_hit;
long int TLB_miss;
long int memory_hit;
long int memory_miss;

void initialize_tcb() {
    currentTCB = (TCB*)pvPortMalloc(sizeof(TCB));
    page_table_init(currentTCB);
}

void uninitialize_tcb() {
    vPortFree(currentTCB);
}

int read_to_memory(int memory_frame, int disk_start_address)
{
	int memory_address, disk_address;
	for (int i = 0; i < page_size; i += 2)
	{
		memory_address = memory_frame * page_size + i;
		disk_address = disk_start_address + i;
		memory[memory_address] = disk[disk_address];
	}
	for (int i = 1; i < page_size; i += 2)
	{
		memory_address = memory_frame * page_size + i;
		disk_address = disk_start_address + i;
		memory[memory_address] = disk[disk_address];
	}
	time_cost += time_disk_access;
	return 1;
}

int write_to_disk(int memory_frame, int disk_start_address)
{
	int memory_address, disk_address;
	for (int i = 0; i < page_size; i += 2)
	{
		memory_address = memory_frame * page_size + i;
		disk_address = disk_start_address + i;
		disk[disk_address] = memory[memory_address];
	}
	for (int i = 1; i < page_size; i += 2)
	{
		memory_address = memory_frame * page_size + i;
		disk_address = disk_start_address + i;
		disk[disk_address] = memory[memory_address];
	}
	time_cost += time_disk_access;
	return 1;
}

void page_table_init(TCB *tcb)
{
	tcb->page_table = (entry*)pvPortMalloc(page_table_size * sizeof(entry));
	for (int i = 0; i < page_table_size; i++)
	{
		tcb->page_table[i].dirty = 0;
		tcb->page_table[i].valid = 0;
		tcb->page_table[i].frame_number = 0;
		tcb->page_table[i].disk_address = start_address + page_size * i;
	}
	for (int i = 0; i < memory_frame_size; i++)
	{
		read_to_memory(i, tcb->page_table[i].disk_address);
		tcb->page_table[i].frame_number = i;
		tcb->page_table[i].valid = 1;
	}
}

int address_map(int virtual_address, enum memory_operation operation)
{
	int page_number = virtual_address / page_size;
	int offset = virtual_address % page_size;
	int physical_address;
#if(1 == useTLB)
	if ((physical_address = TLB_search(virtual_address, operation)) != -1)
	{
		TLB_hit++;
		memory_hit++;
		return physical_address;
	}
	TLB_miss++;
#endif
	
	time_cost += time_memory_access;
	if (!currentTCB->page_table[page_number].valid)
	{
		pageFault((currentTCB->page_table) + page_number, page_number);
		memory_miss++;
	}
	else memory_hit++;
	physical_address = currentTCB->page_table[page_number].frame_number * page_size + offset;
#if(0 == ReplacementStrategy)
	pMovetoFirst(LRU_list, currentTCB->page_table[page_number].frame_number);
#endif
	if (operation == write)
	{
		currentTCB->page_table[page_number].dirty = 1;
	}
#if(1 == useTLB)
	TLB_update(page_number, currentTCB->page_table[page_number].frame_number);
#endif
	return physical_address;
	
}

void pageFault(entry * faultPage, int page_number)
{
#if(0 == ReplacementStrategy)
	LINKNODE endNode = GetEndNode(LRU_list);
#endif
#if(1 == ReplacementStrategy)
	LINKNODE endNode = FIFO_list;
#endif
#if(1 == useTLB)
	if (endNode->task_belonging == currentTCB)
	{
		for (int i = 0; i < TLB_size; i++)
		{
			if (TLB[i].valid == 1 && TLB[i].page_number == endNode->page_number)
			{
				TLB[i].valid = 0;
				if (TLB[i].dirty == 1)
				{
					((currentTCB->page_table) + (endNode->page_number))->dirty = 1;
				}
				break;
			}
		}
	}
#endif
	
	if (((endNode->task_belonging->page_table) + (endNode->page_number))->dirty == 1)
	{
		int disk_address_out = ((endNode->task_belonging->page_table) + (endNode->page_number))->disk_address;
		write_to_disk(endNode->frame_number, disk_address_out);
	}
	((endNode->task_belonging->page_table) + (endNode->page_number))->valid = 0;
	
	int disk_address_in = faultPage->disk_address;
	read_to_memory(endNode->frame_number, disk_address_in);
	faultPage->dirty = 0;
	faultPage->valid = 1;
	faultPage->frame_number = endNode->frame_number;
	
	endNode->task_belonging = currentTCB;
	endNode->page_number = page_number;
#if(1 == ReplacementStrategy)
	FIFO_list = FIFO_list->next;
#endif
}

void LRU_list_init(LINKNODE *list)
{
	LINKNODE s;
	pInitList(list);
	for (int i = 0; i < memory_frame_size; i++)
	{
		s = (LINKNODE)pvPortMalloc(sizeof(NODE));
		s->task_belonging = currentTCB;
		s->page_number = i;
		s->frame_number = i;
		pInsertElem(list, s, 1);
	}
}

int read_memory(int virtual_address)
{
	int physical_address = address_map(virtual_address, read);
	int data = memory[physical_address];
	time_cost += time_cach_access;
	return data;
}

void write_memory(int virtual_address, int data)
{
	int physical_address = address_map(virtual_address, write);
	memory[physical_address] = data;
	time_cost += time_cach_access;
}



int TLB_search(int virtual_address, enum memory_operation operation)
{
	int page_number = virtual_address / page_size;
	int offset = virtual_address % page_size;
	int physical_address = -1;
	for (int i = 0; i < TLB_size; i++)
	{
		if (TLB[i].page_number == page_number && TLB[i].valid == 1)
		{
			TLB[i].ref = 1;
			if (operation == write)
			{
				TLB[i].dirty = 1;
			}
			
			physical_address = TLB[i].frame_number * page_size + offset;
			break;
		}
	}
	time_cost += time_TLB_access;
	return physical_address;
}

int TLB_update(int page_number, int frame_number)
{
	for (int i = 0; i < TLB_size; i++)
	{
		if (TLB[i].valid == 0)
		{
			TLB[i].page_number = page_number;
			TLB[i].frame_number = frame_number;
			TLB[i].dirty = 0;
			TLB[i].ref = 1;
			TLB[i].valid = 1;
			return 1;
		}
	}
	for (int i = 0; i < TLB_size; i++)
	{
		if (TLB[i].valid == 1 && TLB[i].ref == 1 && i != TLB_size - 1)
		{
			TLB[i].ref = 0;
		}
		else
		{
			if (TLB[i].valid == 1 && TLB[i].dirty == 1)
			{
				((currentTCB->page_table) + (TLB[i].page_number))->dirty = 1;
			}
			TLB[i].page_number = page_number;
			TLB[i].frame_number = frame_number;
			TLB[i].dirty = 0;
			TLB[i].ref = 1;
			TLB[i].valid = 1;
			break;
		}
	}
	return 1;
}

void write_back()
{
	for (int i = 0; i < TLB_size; i++)
	{
		if (TLB[i].valid == 1 && TLB[i].dirty == 1)
		{
			((currentTCB->page_table) + (TLB[i].page_number))->dirty = 1;
		}
	}
	
	for (int i = 0; i < memory_frame_size; i++)
	{
		if (((currentTCB->page_table) + i)->valid == 1 && ((currentTCB->page_table) + i)->dirty == 1)
		{
			write_to_disk(((currentTCB->page_table) + i)->frame_number, ((currentTCB->page_table) + i)->disk_address);
		}
	}
	
}

void FIFO_list_init(LINKNODE *list)
{
	LINKNODE s, p;
	pInitList(list);
	for (int i = 0; i < memory_frame_size; i++)
	{
		s = (LINKNODE)pvPortMalloc(sizeof(NODE));
		s->task_belonging = currentTCB;
		s->page_number = i;
		s->frame_number = i;
		pInsertElem(list, s, 1);
	}
	p = GetEndNode(*list);
	p->next = (*list)->next;
	*list = (*list)->next;
}

void initialize_list() {
    #if ReplacementStrategy == 0
    LRU_list_init(&LRU_list);
    #elif ReplacementStrategy == 1
    FIFO_list_init(&FIFO_list);
    #endif
}
