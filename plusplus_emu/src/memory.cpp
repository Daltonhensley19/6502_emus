#include "../include/memory.h"

EmuMemory initalize_memory()
{
  EmuMemory mem{};

  for (auto item : mem.zero_page.buffer)
    item = 0;

  for (auto item : mem.stack.buffer)
    item = 0;

  for (auto item : mem.gen_purpose.buffer)
    item = 0;

  return mem;
}

std::ostream& operator<<(std::ostream& os, const EmuMemory& other)
{
  std::cout << "[ZERO_PAGE MEMORY]: { \n";
  for (const auto item : other.zero_page.buffer)
  {
    std::cout << item;
  }
  std::cout << "}\n\n";

  std::cout << "[STACK MEMORY]: { \n";
  for (const auto item : other.stack.buffer)
  {
    std::cout << item;
  }
  std::cout << "}\n\n";

  std::cout << "[GENERAL_PURPOSE MEMORY]: { \n";
  for (const auto item : other.gen_purpose.buffer)
  {
    std::cout << item;
  }
  std::cout << "}\n\n";

  return os;
}
