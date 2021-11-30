#include "types.h"
#include "hypermain.h"
#include "framebuffer.h"
#include "logo.h"

void hypervisor_entry (boot_args *args) {
	uint32_t *vram = (uint32_t *)args->Video.v_baseAddr;
	for (int x = 0; x < args->Video.v_width; x++) {
		for (int y = 0; y < args->Video.v_height; y++) {
			if (y < 1080 && x < 1920) {
				vram[(y * args->Video.v_width) + x] = pack_color(
															pacman_logo[3 * ((y * 1920) + x)+0],
															pacman_logo[3 * ((y * 1920) + x)+1],
															pacman_logo[3 * ((y * 1920) + x)+2]
														);
			}
		}
	}
	return;
}
