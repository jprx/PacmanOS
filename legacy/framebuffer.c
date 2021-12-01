#include "framebuffer.h"

#define R_PACK_SHIFT_AMOUNT ((22))
#define G_PACK_SHIFT_AMOUNT ((12))
#define B_PACK_SHIFT_AMOUNT ((2))
#define BITMASK10 ((0x3FF))

/*
 * pack_color
 * Takes a red, green, and blue value and computes the iBoot compliant 10-bit color value
 */
uint32_t pack_color(uint32_t r, uint32_t g, uint32_t b) {
	return (r << R_PACK_SHIFT_AMOUNT) |
			(g << G_PACK_SHIFT_AMOUNT) |
			(b << B_PACK_SHIFT_AMOUNT);
}

/*
 * get_channel
 * Returns the appropriate color value for a particular packed color
 */
uint32_t get_r(uint32_t packed) {
	return (packed >> R_PACK_SHIFT_AMOUNT) & BITMASK10;
}

uint32_t get_g(uint32_t packed) {
	return (packed >> G_PACK_SHIFT_AMOUNT) & BITMASK10;
}

uint32_t get_b(uint32_t packed) {
	return (packed >> B_PACK_SHIFT_AMOUNT) & BITMASK10;
}
