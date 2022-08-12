module Registers
  ( StatusRegs
  , Registers
  , initializeRegs
  ) where

import Data.Word (Word16, Word8)

data StatusRegs =
  StatusRegs
    { carryR :: Word8
    , zeroR :: Word8
    , interruptR :: Word8
    , decimalR :: Word8
    , breakR :: Word8
    , unusedR :: Word8
    , negativeR :: Word8
    }
  deriving (Show)

data Registers =
  Registers
    { pc :: Word16
    , acc :: Word8
    , x :: Word8
    , y :: Word8
    , sp :: Word8
    , sr :: StatusRegs
    }
  deriving (Show)

initializeRegs :: Registers
initializeRegs =
  Registers {pc = 0, acc = 0, x = 0, y = 0, sp = 0, sr = initalizeSr}
  where
    initalizeSr =
      StatusRegs
        { carryR = 0
        , zeroR = 0
        , interruptR = 0
        , decimalR = 0
        , breakR = 0
        , unusedR = 1
        , negativeR = 0
        }
