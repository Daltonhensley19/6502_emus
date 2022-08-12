#include "../include/types.h"
#include <cstdint>

struct StatusRegs
{
  u8 C      : 1;
  u8 Z      : 1;
  u8 I      : 1;
  u8 D      : 1;
  u8 B      : 1;
  u8 UNUSED : 1;
  u8 V      : 1;
  u8 N      : 1;
};

struct Registers
{
  u16 pc;
  u8 acc;
  u8 x;
  u8 y;
  u8 sp;
  StatusRegs sr;
};
