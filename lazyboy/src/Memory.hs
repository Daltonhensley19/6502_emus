module Memory
  ( EmuMemory(..)
  , initalizeEmuMemory
  ) where

import Data.Int (Int8)
import qualified Data.Vector as V
import Data.Word (Word16)

data EmuMemory =
  EmuMemory
    { zeroPage :: V.Vector Int8
    , stack :: V.Vector Int8
    , generalPurpose :: V.Vector Int8
    }
  deriving (Show)

initalizeEmuMemory :: EmuMemory
initalizeEmuMemory =
  EmuMemory {zeroPage = zeroMem, stack = stackMem, generalPurpose = genMem}
  where
    zeroMem = V.replicate (0x0FFF - 0x0000) 0
    stackMem = V.replicate (0x01FF - 0x0100) 0
    genMem = V.replicate (0xFFFF - 0x0200) 0
