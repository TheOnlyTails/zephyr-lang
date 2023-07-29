{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards #-}

module Parser.AST where

import Control.Exception
import Data.List
import Data.Map (Map)
import Data.Map qualified as Map
import Data.Maybe (fromMaybe)
import Data.Text (Text)
import Data.Text qualified as Text
import Utils (Name)

data AuraValue
  = IntValue Integer
  | FloatValue Double
  | CharValue Char
  | StringValue Text
  | BooleanValue Bool
  | ListValue [AuraValue]
  | MapValue (Map AuraValue AuraValue)
  | TupleValue [AuraValue]
  | ObjectValue (Map Name AuraValue)
  | VoidValue
  | ClosureValue (Map Name AuraType) AuraExpression AuraType

data AuraExpression
  = IntExpr Integer
  | FloatExpr Double
  | CharExpr Char
  | StringExpr Text
  | BooleanExpr Bool
  | ListExpr [AuraExpression]
  | MapExpr (Map AuraExpression AuraExpression)
  | TupleExpr [AuraExpression]
  | ObjectExpr (Map Name (AuraExpression, Bool))
  | ClosureExpr
      { closureParams :: Map Name AuraType
      , closureRetType :: Maybe AuraType
      , closureBody :: AuraExpression
      }
  | IdentifierExpr
      { identName :: Name
      }
  | CallExpr
      { callFunc :: AuraExpression
      , callArgs :: [AuraExpression]
      }
  | BinaryOpExpr
      { binOp :: BinaryOp
      , lhs :: AuraExpression
      , rhs :: AuraExpression
      }
  | PrefixOpExpr PrefixOp AuraExpression
  | PostfixOpExpr PostfixOp AuraExpression
  | LetExpr
      { letName :: Name
      , letMutable :: Bool
      , letValue :: AuraExpression
      , letType :: Maybe AuraType
      }
  | IfExpr
      { ifCond :: AuraExpression
      , ifThen :: AuraExpression
      , ifElse :: Maybe AuraExpression
      }
  | WhileExpr
      { whileCond :: AuraExpression
      , whileBody :: AuraExpression
      }
  | ForExpr
      { forVariableName :: Name
      , forIterable :: AuraExpression
      , forBody :: AuraExpression
      }
  | -- for idents - if doesn't exist, import everything
    -- if it does, each ident may or may not have an alias
    ImportExpr String (Maybe (Map Name (Maybe Name)))
  | CastExpr AuraExpression AuraType
  | TypeCheckExpr AuraExpression AuraType
  | CodeBlockExpr [AuraExpression]
  deriving (Eq, Ord)

instance Show AuraExpression where
  show expr = case expr of
    IntExpr val -> "Int: " ++ show val
    FloatExpr val -> "Float: " ++ show val
    CharExpr val -> "Char: " ++ show val
    StringExpr val -> "String: " ++ show val
    BooleanExpr val -> "Boolean: " ++ (Text.unpack . Text.toLower . Text.pack $ show val)
    ListExpr items -> "List: #[" ++ intercalate ", " (map show items) ++ "]"
    MapExpr entries -> "Map: #{" ++ showEntries entries ++ "}"
    TupleExpr items -> "Tuple: #(" ++ intercalate ", " (map show items) ++ ")"
    ObjectExpr fields -> "Object: @{" ++ showEntries fields ++ "}"
    ClosureExpr {closureParams, closureRetType, closureBody} ->
      "Closure: ("
        ++ showEntries closureParams
        ++ ")"
        ++ maybe "" ((": " ++) . show) closureRetType
        ++ " -> "
        ++ show closureBody
    IdentifierExpr {identName, ..} -> "Ident: " ++ Text.unpack identName
    CallExpr {callFunc, callArgs} ->
      "Call: ("
        ++ show callFunc
        ++ ")("
        ++ intercalate ", " (map show callArgs)
        ++ ")"
    BinaryOpExpr {binOp, lhs, rhs} -> "Bin op: (" ++ show lhs ++ ") " ++ show binOp ++ " (" ++ show rhs ++ ")"
    PrefixOpExpr op expr -> "Prefix op: " ++ show op ++ "(" ++ show expr ++ ")"
    PostfixOpExpr op expr -> "Postfix op: (" ++ show expr ++ ")" ++ show op
    LetExpr {letName, letMutable, letType, letValue} ->
      "let "
        ++ (if letMutable then "mut " else "")
        ++ Text.unpack letName
        ++ ": "
        ++ show letType
        ++ " = "
        ++ show letValue
    IfExpr {ifCond, ifThen, ifElse} ->
      "if ("
        ++ show ifCond
        ++ ") "
        ++ show ifThen
        ++ "\nelse "
        ++ show ifElse
    WhileExpr {whileCond, whileBody} -> "while (" ++ show whileCond ++ ") " ++ show whileBody
    ForExpr {forVariableName, forIterable, forBody} ->
      "for (" ++ Text.unpack forVariableName ++ " in " ++ show forIterable ++ ") " ++ show forBody
    ImportExpr path idents ->
      "import \""
        ++ path
        ++ "\""
        ++ maybe
          ""
          ( \idents ->
              " with ("
                ++ ( intercalate ", "
                      . map (\(ident, alias) -> Text.unpack ident ++ maybe "" ((" as " ++) . Text.unpack) alias)
                   )
                  (Map.assocs idents)
                ++ ")"
          )
          idents
    CastExpr expr to -> "Cast: " ++ show expr ++ " as " ++ show to
    TypeCheckExpr expr is -> "Type check: " ++ show expr ++ " is " ++ show is
    CodeBlockExpr body -> "Block: {\n" ++ (intercalate "\n" . map (("\t" ++) . show) $ body) ++ "\n}"

showEntries :: (Show k, Show v) => Map k v -> String
showEntries entries =
  intercalate
    ", "
    (map (\(k, v) -> show k ++ " => " ++ show v) (Map.assocs entries))

data AuraType
  = IntType
  | FloatType
  | CharType
  | StringType
  | BooleanType
  | VoidType
  | ListType AuraType
  | MapType AuraType AuraType
  | TupleType [AuraType]
  | ObjectType (Map Text AuraType)
  | ClosureType
      { params :: [AuraType]
      , returnType :: AuraType
      }
  | UnionType [AuraType]
  deriving (Eq, Ord)

instance Show AuraType where
  show typeDef = case typeDef of
    IntType -> "int"
    FloatType -> "float"
    CharType -> "char"
    StringType -> "string"
    BooleanType -> "boolean"
    VoidType -> "void"
    ListType elem -> "#[" ++ show elem ++ "]"
    MapType key value -> "#{" ++ show key ++ ": " ++ show value ++ "}"
    TupleType items -> "#(" ++ (intercalate ", " . map show) items ++ ")"
    ObjectType fields -> "#{" ++ showEntries fields ++ "}"
    ClosureType {params, returnType} ->
      "(" ++ (intercalate ", " . map show) params ++ ") -> " ++ show returnType
    UnionType types -> intercalate " | " . map show $ types

assignableTo :: AuraType -> AuraType -> Bool
assignableTo from to =
  (from == to)
    || ( case (from, to) of
          (_, VoidType) -> False
          (IntType, FloatType) -> True
          (FloatType, IntType) -> True
          (StringType, ListType CharType) -> True
          (ListType CharType, StringType) -> True
          (UnionType aTypes, UnionType bTypes) -> all (\a -> any (\b -> a `assignableTo` b) bTypes) aTypes
          (_, UnionType types) -> from `elem` types
          (MapType mapKey mapValue, ObjectType fields) -> mapKey `assignableTo` StringType && all (mapValue `assignableTo`) fields
          (ObjectType fields, MapType mapKey mapValue) -> mapKey `assignableTo` StringType && all (`assignableTo` mapValue) fields
          (TupleType items, ListType elemType) -> all (`assignableTo` elemType) items
          (ListType elemType, TupleType items) -> all (elemType `assignableTo`) items
          (ListType from, ListType to) -> from `assignableTo` to
          (MapType fromK fromV, MapType toK toV) -> fromK `assignableTo` toK && fromV `assignableTo` toV
          (TupleType fromItems, TupleType toItems) -> all (uncurry assignableTo) (zip fromItems toItems)
          (ObjectType fromFields, ObjectType toFields) ->
            all
              (\(k, v) -> v `assignableTo` fromMaybe VoidType (k `Map.lookup` toFields))
              (Map.assocs fromFields)
       )

data BinaryOp
  = Assign
  | Plus
  | PlusAssign
  | Minus
  | MinusAssign
  | Times
  | TimesAssign
  | Divide
  | DivideAssign
  | Modulus
  | ModulusAssign
  | Power
  | PowerAssign
  | Equals
  | NotEquals
  | LessThan
  | LessThanEquals
  | GreaterThan
  | GreaterThanEquals
  | And
  | Or
  | In
  | NotIn
  deriving (Eq, Ord)

assignmentOps :: [BinaryOp]
assignmentOps = [Assign, PlusAssign, MinusAssign, TimesAssign, DivideAssign, ModulusAssign, PowerAssign]

instance Show BinaryOp where
  show op = case op of
    Assign -> "="
    Plus -> "+"
    PlusAssign -> "+="
    Minus -> "-"
    MinusAssign -> "-="
    Times -> "*"
    TimesAssign -> "*="
    Divide -> "/"
    DivideAssign -> "/="
    Modulus -> "%"
    ModulusAssign -> "%="
    Power -> "**"
    PowerAssign -> "**="
    Equals -> "=="
    NotEquals -> "!="
    LessThanEquals -> "<="
    LessThan -> "<"
    GreaterThan -> ">"
    GreaterThanEquals -> ">="
    And -> "&&"
    Or -> "||"
    In -> "in"
    NotIn -> "!in"

data PrefixOp
  = Negate
  | Not
  deriving (Eq, Ord)

instance Show PrefixOp where
  show op = case op of
    Negate -> "-"
    Not -> "!"

data PostfixOp
  = Increment
  | Decrement
  deriving (Eq, Ord)

instance Show PostfixOp where
  show op = case op of
    Increment -> "++"
    Decrement -> "--"
