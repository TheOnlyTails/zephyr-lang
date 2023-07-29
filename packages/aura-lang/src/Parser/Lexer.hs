{-# LANGUAGE OverloadedLists #-}
{-# LANGUAGE OverloadedStrings #-}

module Parser.Lexer where

import Control.Monad.Combinators
import Data.List
import Data.Set qualified as Set
import Data.Text (Text)
import Data.Text qualified as Text
import Data.Void (Void)
import Parser.AST (AuraExpression)
import Text.Megaparsec
import Text.Megaparsec.Char
import Text.Megaparsec.Char.Lexer qualified as L

type Parser = Parsec Void Text

reservedIdents :: [Text]
reservedIdents =
  [ "true"
  , "false"
  , "let"
  , "mut"
  , "if"
  , "while"
  , "for"
  , "else"
  , "import"
  , "with"
  , ""
  ]

charLiteral :: Parser Char
charLiteral = lexeme $ between (char '\'') (char '\'' <?> "end of character") charLetter

stringLiteral :: Parser Text
stringLiteral = lexeme $ Text.pack <$> (char '\"' *> manyTill charLetter (char '\"' <?> "end of string"))

charLetter :: Parser Char
charLetter =
  try charUnicode
    <|> try (string "\\n" >> return '\n')
    <|> try (string "\\t" >> return '\t')
    <|> try (string "\\r" >> return '\r')
    <|> try (string "\\b" >> return '\b')
    <|> try (string "\\'" >> return '\'')
    <|> try (string "\\\"" >> return '"')
    <|> try (satisfy (\c -> c /= '\\' && c /= '"' && c /= '\''))

charUnicode :: Parser Char
charUnicode = do
  string "\\u"
  digits <- count 4 hexDigitChar
  return $ read ("'\\x" ++ digits ++ "'")

identifier :: Parser Text
identifier = lexeme $ do
  first <- char '_' <|> letterChar
  rest <- many $ char '_' <|> alphaNumChar
  let result = Text.pack $ first : rest

  if result `elem` reservedIdents
    then fail $ "\"" ++ (first : rest) ++ "\" is a reserved identifier."
    else return result

parens :: Parser a -> Parser a
parens = between (char '(') (char ')')

brackets :: Parser a -> Parser a
brackets = between (char '[') (char ']')

braces :: Parser a -> Parser a
braces = between (char '{') (char '}')

angles :: Parser a -> Parser a
angles = between (char '<') (char '>')

commaSep :: Parser a -> Parser [a]
commaSep p = p `sepBy` symbol ","

lexeme :: Parser a -> Parser a
lexeme = L.lexeme spaceConsumer

symbol :: Text -> Parser Text
symbol = L.symbol spaceConsumer

spaceConsumer :: Parser ()
spaceConsumer =
  L.space
    space1
    (L.skipLineComment "//")
    (L.skipBlockComment "/*" "*/")
