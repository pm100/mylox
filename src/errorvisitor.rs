use crate::antlr::{loxparser::LoxParserContextType, loxvisitor::LoxVisitorCompat};
use antlr_rust::atn_config_set::ATNConfigSet;
use antlr_rust::dfa::DFA;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::{
    error_listener::ErrorListener,
    tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat},
};
use bit_set::BitSet;

/*

    Errdetectvisitor is used to detect that a parsing error has occurred.
    It seems impossible to get the error from the parser or from the ErrorHandler
    But the parser does insert ErrorNodes into the tree, so we can detect those.
    So this visitor makes a pass over the tree and if it finds an ErrorNode it returns it
    Run before the vistor that does the actual work, so that the error is detected before

*/
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

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        // once an error is detected keep propagting it
        if aggregate == ErrVal::Empty {
            next
        } else {
            aggregate
        }
    }
    fn visit_error_node(&mut self, node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        ErrVal::Error(node.get_text())
    }
}
impl LoxVisitorCompat<'_> for ErrDetectVisit {
    // this takes the default implementation
    // which just visits the entire tree
}

// This is the error listerner that is attached to the parser
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
        println!("a line {}:{} {}", line, column, msg);
    }

    fn report_ambiguity(
        &self,
        _recognizer: &T,
        _dfa: &DFA,
        start_index: isize,
        stop_index: isize,
        _exact: bool,
        _ambig_alts: &BitSet,
        _configs: &ATNConfigSet,
    ) {
        println!("b error {} {}", start_index, stop_index   );
    }

    fn report_attempting_full_context(
        &self,
        _recognizer: &T,
        _dfa: &DFA,
        start_index: isize,
        stop_index: isize,
        _conflicting_alts: &BitSet,
        _configs: &ATNConfigSet,
    ) {
        println!("c error {} {}", start_index, stop_index   );
    }

    fn report_context_sensitivity(
        &self,
        _recognizer: &T,
        _dfa: &DFA,
        start_index: isize,
        stop_index: isize,
        _prediction: isize,
        _configs: &ATNConfigSet,
    ) {
        println!("d error {} {}", start_index, stop_index   );
    }
}
