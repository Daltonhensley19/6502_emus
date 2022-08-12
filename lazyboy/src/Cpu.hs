module Cpu
  ( AddressingMode
  , Cpu6502
  , initalizeCpu
  ) where

import Data.Word (Word16, Word8)
import Memory (EmuMemory, initalizeEmuMemory)
import Registers (Registers, initializeRegs)

data AddressingMode
  = Accumulator
  | Implied
  | Immediate Word8
  | Absolute Word16
  | ZPage Word8
  | ZPageX Word8 Word8
  | ZPageY Word8 Word8
  | Relative Word8
  | AbsoluteIndirect Word16
  | AbsoluteIndexX Word16 Word8
  | AbsoluteIndexY Word16 Word8
  | Indirect Word16
  | IndirectX Word8 Word8
  | IndirectY Word8 Word8
  deriving (Show)

data Cpu6502 =
  Cpu6502
    { ram :: EmuMemory
    , regs :: Registers
    }
  deriving (Show)

initalizeCpu :: Cpu6502
initalizeCpu = Cpu6502 {ram = initalizeEmuMemory, regs = initializeRegs}
