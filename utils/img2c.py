#!/usr/bin/env python3
# Joseph Ravichandran
# img2c from the PwnyOS 2 project
import sys
from PIL import Image

def main():
    if len(sys.argv) != 3:
        print("Usage: img2c.py [image] [output]")
        exit(-1)

    with open (sys.argv[2], "w") as output_f:
        img = Image.open(sys.argv[1])
        width, height = img.size

        output_f.write(f"const uint8_t ic_background[{width}*{height}*3] = ")
        output_f.write("{\n")
        for y in range(height):
            for x in range(width):
                coord = x, y
                r, g, b, a = img.getpixel(coord)

                output_f.write(hex(r))
                output_f.write(",")
                output_f.write(hex(g))
                output_f.write(",")
                output_f.write(hex(b))

                if not (y == (height - 1) and x == (width - 1)):
                    output_f.write(",")
            output_f.write("\n")
        output_f.write("};")

if __name__ == "__main__":
    main()
