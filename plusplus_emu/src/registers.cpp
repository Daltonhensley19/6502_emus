#include "../include/registers.h"
#include "../include/types.h"
#include <cstdint>

StatusRegs initalize_status_regs()
{
  StatusRegs sregs{
    .C = 0, .Z = 0, .I = 0, .D = 0, .B = 0, .UNUSED = 1, .V = 0, .N = 0};

  return sregs;
}

Registers initalize_regs()
{
  Registers regs{
    .pc = 0, .acc = 0, .x = 0, .y = 0, .sp = 0, .sr = initalize_status_regs()};

  return regs;
}

