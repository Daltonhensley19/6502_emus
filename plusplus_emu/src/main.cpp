#include "../include/memory.h"

int main()
{
  EmuMemory mem = initalize_memory();

  std::cout << mem;

  return 0;
}
