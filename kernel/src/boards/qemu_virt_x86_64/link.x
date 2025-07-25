MEMORY
{
  ram      : ORIGIN = 0x100000, LENGTH = 0x1000000
}

_stack_start = ORIGIN(ram) + LENGTH(ram);