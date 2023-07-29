{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards #-}

module Parser where

import Control.Exception (throw)
import Control.Monad qualified as Monad
import Control.Monad.Combinators.Expr qualified as Expr
import Data.Bifunctor (bimap)
import Data.Foldable (traverse_)
import Data.Functor.Identity (Identity)
import Data.Map qualified as Map
import Data.Maybe (fromMaybe, isJust, listToMaybe)
import Data.Text (Text)
import Data.Text qualified as Text
import Parser.AST
import Parser.Error
import Parser.Lexer (Parser)
import Parser.Lexer qualified as Lexer
import Text.Megaparsec
import Text.Megaparsec.Char.Lexer qualified as L
import Utils

type ParserEnv = Map.Map Name (AuraType, Bool)

expr :: Parser AuraExpression
expr = Expr.makeExprParser expression operations
 where
  expression =
    choice
      [ Lexer.parens expr
      , try float
      , try integer
      , char
      , string
      , true
      , false
      , try list
      , try mapLit
      , try tuple
      , object
      , closure
      , try call
      , identifier
      , letExpr
      , ifExpr
      , whileExpr
      ]

  -- simple literals
  integer = IntExpr <$> Lexer.lexeme L.decimal
  float = FloatExpr <$> Lexer.lexeme L.float
  char = CharExpr <$> Lexer.charLiteral
  string = StringExpr <$> Lexer.stringLiteral
  true = Lexer.symbol "true" >> return (BooleanExpr True)
  false = Lexer.symbol "false" >> return (BooleanExpr False)

  -- collection literals
  list = ListExpr <$> listLiteral (Lexer.commaSep expr)
  mapLit = MapExpr <$> mapLiteral (Map.fromList <$> Lexer.commaSep (entry expr expr ":"))
  tuple = TupleExpr <$> tupleLiteral (Lexer.commaSep expr)
  object =
    ObjectExpr
      <$> objectLiteral
        ( Map.fromList
            <$> Lexer.commaSep
              ( do
                  hasMut <- optional (Lexer.symbol "mut")
                  let isMutable = isJust hasMut
                  name <- Lexer.identifier
                  Lexer.symbol ":"
                  value <- expr

                  return (name, (value, isMutable))
              )
        )

  closure = do
    params <- Map.fromList <$> Lexer.parens (Lexer.commaSep (entry Lexer.identifier typeAnn ":"))
    retType <- optional . Lexer.lexeme $ Lexer.symbol ":" >> typeAnn
    Lexer.symbol "->"
    body <- expr

    return
      ClosureExpr
        { closureParams = params
        , closureBody = body
        , closureRetType = retType
        }

  identifier = IdentifierExpr <$> Lexer.identifier

  call = do
    func <-
      try identifier
        <|> try call
        <|> try (Lexer.parens expr)
        <?> "Cannot call this type of expression. (help: add parentheses)"
    params <- Lexer.parens $ Lexer.commaSep expr

    return
      CallExpr
        { callFunc = func
        , callArgs = params
        }

  opTerm =
    try float
      <|> integer
      <|> char
      <|> string
      <|> list
      <|> object
      <|> tuple
      <|> mapLit
      <|> try call
      <|> try identifier
      <|> Lexer.parens expr
      <?> "Cannot operate this type of expression. (help: add parentheses)"

  letExpr = do
    Lexer.symbol "let"
    mutable <- isJust <$> optional (Lexer.symbol "mut")
    name <- Lexer.identifier
    annotation <- optional $ try $ Lexer.symbol ":" >> typeAnn

    Lexer.symbol "="
    initialValue <- expr

    return
      LetExpr
        { letName = name
        , letMutable = mutable
        , letType = annotation
        , letValue = initialValue
        }

  ifExpr = do
    Lexer.symbol "if"
    cond <- Lexer.parens expr
    ifThen <- expr
    ifElse <- optional (Lexer.symbol "else" >> expr)

    return
      IfExpr
        { ifCond = cond
        , ifThen
        , ifElse
        }

  whileExpr = do
    Lexer.symbol "while"
    cond <- Lexer.parens expr
    body <- expr

    return
      WhileExpr
        { whileCond = cond
        , whileBody = body
        }

typeOf :: ParserEnv -> AuraExpression -> AuraType
typeOf env expr = case expr of
  IntExpr _ -> IntType
  FloatExpr _ -> FloatType
  CharExpr _ -> CharType
  StringExpr _ -> StringType
  BooleanExpr _ -> BooleanType
  ListExpr items -> ListType $ maybe VoidType (typeOf env) $ listToMaybe items
  MapExpr entries ->
    uncurry MapType $
      maybe (VoidType, VoidType) (bimap (typeOf env) (typeOf env)) $
        listToMaybe (Map.assocs entries)
  TupleExpr items -> TupleType $ map (typeOf env) items
  ObjectExpr fields -> ObjectType $ Map.map (typeOf env . fst) fields
  ClosureExpr {closureParams, closureRetType, closureBody} ->
    ClosureType
      { params = map snd (Map.assocs closureParams)
      , returnType = fromMaybe (typeOf env closureBody) closureRetType
      }
  IdentifierExpr {identName} -> maybe (throw $ UnknownIdent identName) fst (identName `Map.lookup` env)
  CallExpr {callFunc} -> case typeOf env callFunc of
    ClosureType {returnType, ..} -> returnType
    _ -> throw $ CallNonClosure callFunc
  BinaryOpExpr {binOp, lhs, rhs} ->
    let lhsType = typeOf env lhs
        rhsType = typeOf env rhs
     in ( case binOp of
            Assign -> rhsType
            Plus -> case (lhsType, rhsType) of
              (IntType, IntType) -> IntType
              (FloatType, FloatType) -> FloatType
              (StringType, StringType) -> StringType
              (ListType a, ListType b) ->
                if a `assignableTo` b
                  then ListType b
                  else
                    if b `assignableTo` a
                      then ListType a
                      else ListType (UnionType [a, b])
            PlusAssign -> case (lhsType, rhsType) of
              (IntType, IntType) -> IntType
              (FloatType, FloatType) -> FloatType
              (StringType, StringType) -> StringType
              (ListType a, ListType b) ->
                if a `assignableTo` b
                  then ListType b
                  else
                    if b `assignableTo` a
                      then ListType a
                      else ListType (UnionType [a, b])
            Minus -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            MinusAssign -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            Times -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            TimesAssign -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            Divide -> FloatType
            DivideAssign -> FloatType
            Modulus -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            ModulusAssign -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            Power -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            PowerAssign -> if lhsType == IntType && rhsType == IntType then IntType else FloatType
            Equals -> BooleanType
            NotEquals -> BooleanType
            LessThan -> BooleanType
            LessThanEquals -> BooleanType
            GreaterThan -> BooleanType
            GreaterThanEquals -> BooleanType
            And -> BooleanType
            Or -> BooleanType
            In -> BooleanType
            NotIn -> BooleanType
        )
  PrefixOpExpr op operand -> case op of
    Negate -> if typeOf env operand == IntType then IntType else FloatType
    Not -> BooleanType
  PostfixOpExpr op operand -> if typeOf env operand == IntType then IntType else FloatType
  LetExpr {letType, letValue, ..} -> fromMaybe (typeOf env letValue) letType
  IfExpr {ifThen, ifElse, ..} -> UnionType [typeOf env ifThen, maybe VoidType (typeOf env) ifElse]
  WhileExpr {..} -> VoidType
  ForExpr {..} -> VoidType
  ImportExpr path idents -> undefined
  CastExpr _ to -> to
  TypeCheckExpr _ _ -> BooleanType
  CodeBlockExpr body -> maybe VoidType (typeOf env) (listToMaybe . reverse $ body)

typeAnn :: Parser AuraType
typeAnn = do
  t <- simpleType
  ts <- many (Lexer.symbol "|" >> typeAnn)

  return $ case ts of
    [] -> t
    _ -> UnionType $ t : ts
 where
  simpleType =
    try intType
      <|> try floatType
      <|> try charType
      <|> try stringType
      <|> try booleanType
      <|> try voidType
      <|> try listType
      <|> try mapType
      <|> try tupleType
      <|> try objectType
      <|> try closureType
  intType = Lexer.symbol "int" >> return IntType
  floatType = Lexer.symbol "float" >> return FloatType
  charType = Lexer.symbol "char" >> return CharType
  stringType = Lexer.symbol "string" >> return StringType
  booleanType = Lexer.symbol "boolean" >> return BooleanType
  voidType = Lexer.symbol "void" >> return VoidType
  listType = ListType <$> listLiteral typeAnn
  mapType = uncurry MapType <$> mapLiteral (entry typeAnn typeAnn ":")
  tupleType = TupleType <$> tupleLiteral (Lexer.commaSep typeAnn)
  objectType =
    ObjectType <$> objectLiteral (Map.fromList <$> Lexer.commaSep (entry Lexer.identifier typeAnn ":"))
  closureType = do
    params <- Lexer.parens (Lexer.commaSep typeAnn)
    Lexer.symbol "->"
    retType <- typeAnn
    return
      ClosureType
        { params = params
        , returnType = retType
        }

listLiteral :: Parser a -> Parser a
listLiteral = between (Lexer.symbol "#[") (Lexer.symbol "]")

mapLiteral :: Parser a -> Parser a
mapLiteral = between (Lexer.symbol "#{") (Lexer.symbol "}")

tupleLiteral :: Parser a -> Parser a
tupleLiteral = between (Lexer.symbol "#(") (Lexer.symbol ")")

objectLiteral :: Parser a -> Parser a
objectLiteral = between (Lexer.symbol "@{") (Lexer.symbol "}")

entry :: Parser a -> Parser b -> Text -> Parser (a, b)
entry key value separator = do
  k <- key
  Lexer.symbol separator
  v <- value
  return (k, v)

binOpCheck :: Map.Map BinaryOp (AuraType -> AuraType -> Bool)
binOpCheck =
  Map.fromList
    [ (Assign, assignableTo)
    ,
      ( Plus
      , \lhs rhs ->
          numOp lhs rhs
            || stringOp lhs rhs
            || listOp lhs rhs
      )
    ,
      ( PlusAssign
      , \lhs rhs ->
          numOp lhs rhs
            || stringOp lhs rhs
            || listOp lhs rhs
      )
    , (Minus, numOp)
    , (MinusAssign, numOp)
    , (Times, numOp)
    , (TimesAssign, numOp)
    , (Divide, numOp)
    , (DivideAssign, numOp)
    , (Modulus, numOp)
    , (ModulusAssign, numOp)
    , (Power, numOp)
    , (PowerAssign, numOp)
    , (Equals, assignableTo)
    , (NotEquals, assignableTo)
    , (LessThan, numOp)
    , (LessThanEquals, numOp)
    , (GreaterThan, numOp)
    , (GreaterThanEquals, numOp)
    , (And, booleanOp)
    , (Or, booleanOp)
    , (In, containOp)
    , (NotIn, containOp)
    ]
 where
  numOp lhs rhs =
    (lhs `assignableTo` IntType && rhs `assignableTo` IntType)
      || (lhs `assignableTo` FloatType && rhs `assignableTo` FloatType)
  listOp lhs rhs = case (elementType lhs, elementType rhs) of
    (Nothing, _) -> False
    (_, Nothing) -> False
    (Just a, Just b) -> a `assignableTo` b
  containOp lhs rhs = case (lhs, elementType rhs) of
    (_, Nothing) -> False
    (a, Just b) -> a `assignableTo` b
  stringOp lhs rhs = lhs `assignableTo` StringType && rhs `assignableTo` StringType
  booleanOp lhs rhs = lhs `assignableTo` BooleanType && rhs `assignableTo` BooleanType

toBinOp :: String -> BinaryOp
toBinOp "=" = Assign
toBinOp "+" = Plus
toBinOp "+=" = PlusAssign
toBinOp "-" = Minus
toBinOp "-=" = MinusAssign
toBinOp "*" = Times
toBinOp "*=" = TimesAssign
toBinOp "/" = Divide
toBinOp "/=" = DivideAssign
toBinOp "%" = Modulus
toBinOp "%=" = ModulusAssign
toBinOp "**" = Power
toBinOp "**=" = PowerAssign
toBinOp "==" = Equals
toBinOp "!=" = NotEquals
toBinOp "<" = LessThan
toBinOp "<=" = LessThanEquals
toBinOp ">" = GreaterThan
toBinOp ">=" = GreaterThanEquals
toBinOp "&&" = And
toBinOp "||" = Or
toBinOp "in" = In
toBinOp "!in" = NotIn
toBinOp str = throw $ UnknownOperator str

toPrefixOp :: String -> PrefixOp
toPrefixOp "!" = Not
toPrefixOp "-" = Negate
toPrefixOp str = throw $ UnknownOperator str

toPostfixOp :: String -> PostfixOp
toPostfixOp "++" = Increment
toPostfixOp "--" = Decrement
toPostfixOp str = throw $ UnknownOperator str

elementType :: AuraType -> Maybe AuraType
elementType container = case container of
  StringType -> Just CharType
  ListType a -> Just a
  MapType key val -> Just $ TupleType [key, val]
  TupleType items -> Just $ UnionType items
  ObjectType fields -> Just $ TupleType [StringType, UnionType $ Map.elems fields]
  _ -> Nothing

binary, binaryRight, binaryNone :: Text -> BinaryOp -> Expr.Operator Parser AuraExpression
binary name op = Expr.InfixL (BinaryOpExpr op <$ Lexer.symbol name)
binaryRight name op = Expr.InfixR (BinaryOpExpr op <$ Lexer.symbol name)
binaryNone name op = Expr.InfixN (BinaryOpExpr op <$ Lexer.symbol name)

prefix :: Text -> PrefixOp -> Expr.Operator Parser AuraExpression
prefix name op = Expr.Prefix (PrefixOpExpr op <$ Lexer.symbol name)

postfix :: Text -> PostfixOp -> Expr.Operator Parser AuraExpression
postfix name op = Expr.Postfix (PostfixOpExpr op <$ Lexer.symbol name)

operations :: [[Expr.Operator Parser AuraExpression]]
operations =
  [
    [ binary "+" Plus
    , binary "-" Minus
    ]
  ,
    [ binary "*" Times
    , binary "/" Divide
    ]
  ,
    [ binaryRight "**" Power
    ]
  ,
    [ prefix "-" Negate
    , prefix "!" Not
    ]
  ]
