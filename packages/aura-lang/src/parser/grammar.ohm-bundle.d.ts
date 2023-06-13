// AUTOGENERATED FILE
// This file was generated from grammar.ohm by `ohm generateBundles`.

import {
  BaseActionDict,
  Grammar,
  IterationNode,
  Node,
  NonterminalNode,
  Semantics,
  TerminalNode
} from 'ohm-js';

export interface AuraActionDict<T> extends BaseActionDict<T> {
  Program?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Expression_closure_call?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode, arg3: IterationNode, arg4: TerminalNode) => T;
  Expression_code_block?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: TerminalNode) => T;
  Expression_let?: (this: NonterminalNode, arg0: TerminalNode, arg1: IterationNode, arg2: NonterminalNode, arg3: IterationNode, arg4: IterationNode, arg5: TerminalNode, arg6: NonterminalNode) => T;
  Expression_if?: (this: NonterminalNode, arg0: TerminalNode, arg1: TerminalNode, arg2: NonterminalNode, arg3: TerminalNode, arg4: NonterminalNode, arg5: IterationNode, arg6: IterationNode) => T;
  Expression_while?: (this: NonterminalNode, arg0: TerminalNode, arg1: TerminalNode, arg2: NonterminalNode, arg3: TerminalNode, arg4: NonterminalNode) => T;
  Expression_for?: (this: NonterminalNode, arg0: TerminalNode, arg1: TerminalNode, arg2: NonterminalNode, arg3: TerminalNode, arg4: NonterminalNode, arg5: TerminalNode, arg6: NonterminalNode) => T;
  Expression_import?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: IterationNode, arg4: IterationNode, arg5: IterationNode, arg6: IterationNode) => T;
  Expression_cast?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode) => T;
  Expression_type_check?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode) => T;
  Expression_unary_op_prefix?: (this: NonterminalNode, arg0: NonterminalNode, arg1: NonterminalNode) => T;
  Expression_unary_op_postfix?: (this: NonterminalNode, arg0: NonterminalNode, arg1: NonterminalNode) => T;
  Expression_binary_op?: (this: NonterminalNode, arg0: NonterminalNode, arg1: NonterminalNode, arg2: NonterminalNode) => T;
  Expression_boolean?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Expression_ident?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Expression_float?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Expression_int?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Expression_char?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Expression_string?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Expression_closure?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode, arg4: TerminalNode, arg5: NonterminalNode) => T;
  Expression_list?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode) => T;
  Expression_map?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode) => T;
  Expression_tuple?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode) => T;
  Expression_object?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode) => T;
  Expression_parens?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: TerminalNode) => T;
  Expression?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  int?: (this: NonterminalNode, arg0: NonterminalNode, arg1: IterationNode) => T;
  float?: (this: NonterminalNode, arg0: NonterminalNode, arg1: IterationNode, arg2: TerminalNode, arg3: NonterminalNode, arg4: IterationNode) => T;
  char?: (this: NonterminalNode, arg0: TerminalNode, arg1: IterationNode | NonterminalNode, arg2: TerminalNode) => T;
  string?: (this: NonterminalNode, arg0: TerminalNode, arg1: IterationNode, arg2: TerminalNode) => T;
  escapeSequence?: (this: NonterminalNode, arg0: TerminalNode, arg1: TerminalNode) => T;
  unicodeChar?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: NonterminalNode, arg3: NonterminalNode, arg4: NonterminalNode) => T;
  identifier?: (this: NonterminalNode, arg0: NonterminalNode, arg1: IterationNode) => T;
  ClosureParam?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode) => T;
  MapEntry?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode) => T;
  ObjectEntry?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode) => T;
  binaryOperator_plus?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_plus_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_minus?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_minus_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_multiply?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_multiply_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_divide?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_divide_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_modulo?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_modulo_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_exponent?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_exponent_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_equals?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_not_equals?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_less_than?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_less_than_equals?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_greater_than?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_greater_than_equals?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_and?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_or?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_in?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_not_in?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator_assign?: (this: NonterminalNode, arg0: TerminalNode) => T;
  binaryOperator?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  unaryPrefixOperator_not?: (this: NonterminalNode, arg0: TerminalNode) => T;
  unaryPrefixOperator_negate?: (this: NonterminalNode, arg0: TerminalNode) => T;
  unaryPrefixOperator?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  unaryPostfixOperator_increment?: (this: NonterminalNode, arg0: TerminalNode) => T;
  unaryPostfixOperator_decrement?: (this: NonterminalNode, arg0: TerminalNode) => T;
  unaryPostfixOperator?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  ImportIdent?: (this: NonterminalNode, arg0: NonterminalNode, arg1: IterationNode, arg2: IterationNode) => T;
  importPath?: (this: NonterminalNode, arg0: TerminalNode, arg1: IterationNode, arg2: TerminalNode) => T;
  Type_union?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  Type_list?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: TerminalNode) => T;
  Type_map?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: TerminalNode, arg3: NonterminalNode, arg4: TerminalNode) => T;
  Type_tuple?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode) => T;
  Type_object?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode) => T;
  Type_closure?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: IterationNode, arg3: TerminalNode, arg4: TerminalNode, arg5: NonterminalNode) => T;
  Type_int?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Type_float?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Type_char?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Type_string?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Type_boolean?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Type_void?: (this: NonterminalNode, arg0: TerminalNode) => T;
  Type_ident?: (this: NonterminalNode, arg0: NonterminalNode, arg1: IterationNode, arg2: IterationNode, arg3: IterationNode, arg4: IterationNode) => T;
  Type_parens?: (this: NonterminalNode, arg0: TerminalNode, arg1: NonterminalNode, arg2: TerminalNode) => T;
  Type?: (this: NonterminalNode, arg0: NonterminalNode) => T;
  ObjectTypeEntry?: (this: NonterminalNode, arg0: NonterminalNode, arg1: TerminalNode, arg2: NonterminalNode) => T;
}

export interface AuraSemantics extends Semantics {
  addOperation<T>(name: string, actionDict: AuraActionDict<T>): this;
  extendOperation<T>(name: string, actionDict: AuraActionDict<T>): this;
  addAttribute<T>(name: string, actionDict: AuraActionDict<T>): this;
  extendAttribute<T>(name: string, actionDict: AuraActionDict<T>): this;
}

export interface AuraGrammar extends Grammar {
  createSemantics(): AuraSemantics;
  extendSemantics(superSemantics: AuraSemantics): AuraSemantics;
}

declare const grammar: AuraGrammar;
export default grammar;
