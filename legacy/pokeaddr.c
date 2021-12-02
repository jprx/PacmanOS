#include "types.h"

void pokeaddr(void) {
	uint32_t *vaddr = (uint32_t *)(0x0000000080000004);
	*vaddr = 0xffffffff;
}
