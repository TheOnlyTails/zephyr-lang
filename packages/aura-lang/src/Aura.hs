{-# LANGUAGE OverloadedStrings #-}
module Aura where

import Data.Map qualified as Map
import Parser
import Text.Megaparsec 
import Parser.AST (AuraExpression)

cli :: IO ()
cli = do
  parseTest expr "(a: int): void -> 1"