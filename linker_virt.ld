ENTRY(_start)
SECTIONS
{
  . = 16M;
  .text : ALIGN(0x1000) { *(.text); }
  .data : ALIGN(0x1000) { *(.data); }
  .bss : ALIGN(0x1000) { *(.bss); }
  .rodata : ALIGN(0x1000) { *(.rodata); }

  . = . + 1M;
  .stack_alignment : ALIGN(0x1000) { }
  _stack_bot = .;

  . = . + 1K;
  .kernel_heap_section : ALIGN(0x1000) { kernel_heap = .; }
  . = . + 16K;
  kernel_heap_end = .;

  kernel_mmap_section : ALIGN(0x200000) { kernel_mmap_begin = .; }
}
