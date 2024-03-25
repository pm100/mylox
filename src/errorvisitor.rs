use antlr_rust::{
    error_listener::ErrorListener,
    tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat},
};

use crate::antlr::{loxparser::LoxParserContextType, loxvisitor::LoxVisitorCompat};
use antlr_rust::recognizer::Recognizer;
#[derive(Default, Debug, PartialEq)]
pub enum ErrVal {
    #[default]
    Empty,
    Error(String),
}
pub struct ErrDetectVisit {
    pub val: ErrVal,
}

impl ErrDetectVisit {
    pub fn new() -> Self {
        Self { val: ErrVal::Empty }
    }
}
impl ParseTreeVisitorCompat<'_> for ErrDetectVisit {
    type Node = LoxParserContextType;
    type Return = ErrVal;
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.val
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        //  unreachable!();
        // println!("Aggregating: {:?} {:?}", _aggregate, next);
        if _aggregate == ErrVal::Empty {
            next
        } else {
            _aggregate
        }
        //next
    }
    fn visit_error_node(&mut self, node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        // println!("Error: {:?}", node.get_text());
        ErrVal::Error(node.get_text())
    }
}
impl LoxVisitorCompat<'_> for ErrDetectVisit {}
pub struct MyErrorListener {}
impl MyErrorListener {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for MyErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as antlr_rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&antlr_rust::errors::ANTLRError>,
    ) {
        println!("line {}:{} {}", line, column, msg);
      
    }

    // fn report_ambiguity(
    //     &self,
    //     _recognizer: &T,
    //     _dfa: &DFA,
    //     _start_index: isize,
    //     _stop_index: isize,
    //     _exact: bool,
    //     _ambig_alts: &BitSet,
    //     _configs: &ATNConfigSet,
    // ) {
    // }

    // fn report_attempting_full_context(
    //     &self,
    //     _recognizer: &T,
    //     _dfa: &DFA,
    //     _start_index: isize,
    //     _stop_index: isize,
    //     _conflicting_alts: &BitSet,
    //     _configs: &ATNConfigSet,
    // ) {
    // }

    // fn report_context_sensitivity(
    //     &self,
    //     _recognizer: &T,
    //     _dfa: &DFA,
    //     _start_index: isize,
    //     _stop_index: isize,
    //     _prediction: isize,
    //     _configs: &ATNConfigSet,
    // ) {
    // }
}
