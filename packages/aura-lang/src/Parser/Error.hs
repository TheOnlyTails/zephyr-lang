module Parser.Error where

import Data.Data (Typeable)
import Data.Text qualified as Text
import GHC.Exception (Exception)
import Parser.AST (AuraExpression, AuraType)
import Utils (Name)

data ParserError
  = CannotAssign AuraType AuraType
  | UnknownIdent Name
  | WrongParamCount Int Int
  | CallNonClosure AuraExpression
  | AssignNotToIdent AuraExpression
  | AssignToImmutable Name
  | WrongOpArgs String AuraType AuraType
  | UnknownOperator String
  deriving (Typeable)

instance Show ParserError where
  show err = case err of
    CannotAssign from to -> "A value of type `" ++ show from ++ "` to a value of type `" ++ show to ++ "`."
    UnknownIdent name -> "Could not find a value named '" ++ Text.unpack name ++ "' in scope."
    WrongParamCount expected found -> "Expected " ++ show expected ++ " arguments for this closure call, found " ++ show found
    CallNonClosure actual -> "Cannot call an expression which is not a closure: " ++ show actual
    AssignNotToIdent actual -> "You may only assign to identifiers: " ++ show actual
    AssignToImmutable name -> "You many not assign to immutable identifier `" ++ show name ++ "`."
    WrongOpArgs opType lhs rhs ->
      "Cannot perform a "
        ++ opType
        ++ " operation on values of types "
        ++ show lhs
        ++ " and "
        ++ show rhs
        ++ "."
    UnknownOperator op -> "The `" ++ op ++ "` operator doesn't exist."

instance Exception ParserError
