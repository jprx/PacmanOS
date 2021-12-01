#!/usr/bin/env python3
# Joseph Ravichandran
# img2rs from the PwnyOS 2 project
import sys
from PIL import Image

def main():
    if len(sys.argv) != 3:
        print("Usage: img2rs.py [image] [output]")
        exit(-1)

    with open (sys.argv[2], "w") as output_f:
        img = Image.open(sys.argv[1])
        width, height = img.size

        output_f.write(f"pub static new_image : [[u32; {width}]; {height}] = ")
        output_f.write("[\n")
        for y in range(height):
            output_f.write("[")
            for x in range(width):
                coord = x, y
                a = 0

                # If this line is causing errors, get rid of the ", a" part:
                # That is because the image you're using doesn't have an alpha channel
                r, g, b, a = img.getpixel(coord)

                output_data = (a << 24) | (r << 16) | (g << 8) | b

                output_f.write(hex(output_data))
                output_f.write(",")

            output_f.write("],\n")
        output_f.write("];")

if __name__ == "__main__":
    main()
