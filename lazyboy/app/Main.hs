module Main where

import Cpu (Cpu6502, initalizeCpu)
import Lib
import Registers (initializeRegs)

main :: IO ()
main = do
  let cpu = initalizeCpu
  print cpu
