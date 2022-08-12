#include <array>
#include <cstdint>
#include <iostream>

// Since `N` is a size, we make sure it's strictly positive!
template<std::uint32_t N>
struct MemoryRegion
{
  std::array<std::int8_t, N> buffer;
};

constexpr std::uint32_t Z_PAGE_SIZE      = 0x00FF - 0x0000;
constexpr std::uint32_t STACK_SIZE       = 0x01FF - 0x0100;
constexpr std::uint32_t GENERAL_MEM_SIZE = 0xFFFF - 0x0200;

struct EmuMemory
{
  MemoryRegion<Z_PAGE_SIZE> zero_page;
  MemoryRegion<STACK_SIZE> stack;
  MemoryRegion<GENERAL_MEM_SIZE> gen_purpose;

  friend std::ostream& operator<<(std::ostream& os, const EmuMemory& other);

};

EmuMemory initalize_memory();
