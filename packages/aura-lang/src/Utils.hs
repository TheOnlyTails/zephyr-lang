module Utils where

import Data.Text (Text)

type Name = Text

enumerate :: [a] -> [(Int, a)]
enumerate = zip [0 ..]
