// auto-generated: "lalrpop 0.20.2"
// sha3: 01df2102c9625208da50ef999335e56207a0a1342f61e32fc9ccb9f6fc8d8951
use std::str::FromStr;
use ordered_float::OrderedFloat;
use lalrpop_util::ParseError;
use crate::parser::ast::Expr;
use crate::error::ZephyrError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::never_loop, clippy::match_single_binding, clippy::needless_raw_string_hashes)]
mod __parse__Expr {

    use std::str::FromStr;
    use ordered_float::OrderedFloat;
    use lalrpop_util::ParseError;
    use crate::parser::ast::Expr;
    use crate::error::ZephyrError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input, 'ctx>
     {
        Variant0(&'input str),
        Variant1(Expr<'ctx>),
        Variant2(OrderedFloat<f64>),
        Variant3(u64),
    }
    const __ACTION: &[i8] = &[
        // State 0
        6, 7, 8, 9, 10,
        // State 1
        0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 5 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -9,
        // State 2
        -8,
        // State 3
        -7,
        // State 4
        -1,
        // State 5
        -4,
        // State 6
        -5,
        // State 7
        -6,
        // State 8
        -3,
        // State 9
        -2,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###"r#"0b[01_]+"#"###,
        r###"r#"0o[0-7_]+"#"###,
        r###"r#"0x[0-9a-fA-F_]+"#"###,
        r###"r#"[1-9][\\d_]*"#"###,
        r###"r#"\\d*\\.\\d+"#"###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'ctx,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'ctx>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'ctx ())>,
    }
    impl<'input, 'ctx> __state_machine::ParserDefinition for __StateMachine<'input, 'ctx>
    where 
    {
        type Location = usize;
        type Error = ZephyrError<'ctx>;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input, 'ctx>;
        type Success = Expr<'ctx>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 5 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'ctx,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'ctx,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> __Symbol<'input, 'ctx>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'ctx,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'ctx>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            8 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub(crate) struct ExprParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for ExprParser { fn default() -> Self { Self::new() } }
    impl ExprParser {
        pub(crate) fn new() -> ExprParser {
            let __builder = super::__intern_token::new_builder();
            ExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            'input,
            'ctx,
        >(
            &self,
            input: &'input str,
        ) -> Result<Expr<'ctx>, __lalrpop_util::ParseError<usize, Token<'input>, ZephyrError<'ctx>>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'ctx,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'ctx,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> Option<Result<Expr<'ctx>,__lalrpop_util::ParseError<usize, Token<'input>, ZephyrError<'ctx>>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                // Float = r#"\\d*\\.\\d+"# => ActionFn(5);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = match super::__action5::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'input,
      'ctx,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>
    ) -> (usize, Expr<'ctx>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
      'ctx,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>
    ) -> (usize, OrderedFloat<f64>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
      'ctx,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
      'ctx,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Expr = Term => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    fn __reduce2<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Int = r#"[1-9][\\d_]*"# => ActionFn(1);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce3<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Int = r#"0b[01_]+"# => ActionFn(2);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce4<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Int = r#"0o[0-7_]+"# => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Int = r#"0x[0-9a-fA-F_]+"# => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce6<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Term = Int => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    fn __reduce7<
        'input,
        'ctx,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input, 'ctx>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'ctx ())>,
    ) -> (usize, usize)
    {
        // Term = Float => ActionFn(7);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
}
#[allow(unused_imports)]
pub(crate) use self::__parse__Expr::ExprParser;
#[rustfmt::skip]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ordered_float::OrderedFloat;
    use lalrpop_util::ParseError;
    use crate::parser::ast::Expr;
    use crate::error::ZephyrError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("(?:(?:0b)[01_]+)", false),
            ("(?:(?:0o)[0-7_]+)", false),
            ("(?:(?:0x)[0-9A-F_a-f]+)", false),
            ("(?:[1-9][0-9_٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𐴰-𐴹𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑥐-𑥙𑱐-𑱙𑵐-𑵙𑶠-𑶩𑽐-𑽙𖩠-𖩩𖫀-𖫉𖭐-𖭙𝟎-𝟿𞅀-𞅉𞋰-𞋹𞓰-𞓹𞥐-𞥙🯰-🯹]*)", false),
            ("(?:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𐴰-𐴹𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑥐-𑥙𑱐-𑱙𑵐-𑵙𑶠-𑶩𑽐-𑽙𖩠-𖩩𖫀-𖫉𖭐-𖭙𝟎-𝟿𞅀-𞅉𞋰-𞋹𞓰-𞓹𞥐-𞥙🯰-🯹]*\\.[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𐴰-𐴹𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑥐-𑥙𑱐-𑱙𑵐-𑵙𑶠-𑶩𑽐-𑽙𖩠-𖩩𖫀-𖫉𖭐-𖭙𝟎-𝟿𞅀-𞅉𞋰-𞋹𞓰-𞓹𞥐-𞥙🯰-🯹]+)", false),
            (r"\s+", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'ctx>, usize),
) -> Expr<'ctx>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u64
{
    __0.replace("_", "").parse().unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u64
{
    u64::from_str_radix(__0[2..].replace("_", ""), 2).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u64
{
    u64::from_str_radix(__0[2..].replace("_", ""), 8).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u64
{
    u64::from_str_radix(__0[2..].replace("_", ""), 16).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<OrderedFloat<f64>,__lalrpop_util::ParseError<usize,Token<'input>,ZephyrError<'ctx>>>
{
    __0.parse().map_err(|e| ParseError::User { error: ZephyrError::ParseError(Box::new(e)) })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, u64, usize),
) -> Expr<'ctx>
{
    Expr::Int(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, OrderedFloat<f64>, usize),
) -> Expr<'ctx>
{
    Expr::Float(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
    'input,
    'ctx,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'ctx>, usize),
) -> Expr<'ctx>
{
    __0
}
#[allow(clippy::type_complexity, dead_code)]

pub(crate)  trait __ToTriple<'input, 'ctx, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ZephyrError<'ctx>>>;
}

impl<'input, 'ctx, > __ToTriple<'input, 'ctx, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ZephyrError<'ctx>>> {
        Ok(value)
    }
}
impl<'input, 'ctx, > __ToTriple<'input, 'ctx, > for Result<(usize, Token<'input>, usize), ZephyrError<'ctx>>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ZephyrError<'ctx>>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
