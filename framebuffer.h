#ifndef FRAMEBUFFER_H
#define FRAMEBUFFER_H

#include "types.h"

/*
 * The iBoot framebuffer uses 10-bit color in the following format:
 *
 * |32      22|21      12|11       2|1  0|
 * +-------------------------------------+
 * |RRRRRRRRRR|GGGGGGGGGG|BBBBBBBBBB| XX |
 * +-------------------------------------+
 *
 * A "packed color" is a color in 10-bit form.
 */

// This header file contains some lightweight testing methods for playing with the framebuffer
// In the future when we move to Rust we will use traits to make this super clean

/*
 * pack_color
 * Takes a red, green, and blue value and computes the iBoot compliant 10-bit color value
 */
uint32_t pack_color(uint32_t r, uint32_t g, uint32_t b);

/*
 * get_channel
 * Returns the appropriate color value for a particular packed color
 */
uint32_t get_r(uint32_t packed);
uint32_t get_g(uint32_t packed);
uint32_t get_b(uint32_t packed);

#endif // FRAMEBUFFER_H
