#include "types.h"
#include "hypermain.h"

void hypervisor_entry (boot_args *args) {
	uint32_t *vram = (uint32_t *)args->Video.v_baseAddr;
	for (int x = 0; x < args->Video.v_width; x++) {
		for (int y = 0; y < args->Video.v_height; y++) {
			vram[(y * args->Video.v_width) + x] = 0xff00ff;
		}
	}
	return;
}
