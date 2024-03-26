use std::path::PathBuf;
use std::{env, fs};

use antlr_rust::tree::Visitable;
use antlr_rust::Parser;

use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use trace::initlog;

use crate::errorvisitor::{ErrDetectVisit, ErrVal, MyErrorListener};
use crate::{
    antlr::{loxlexer::LoxLexer, loxparser::LoxParser},
    interpvisitor::InterpVisit,
};
mod antlr {
    pub mod loxlexer;
    pub mod loxlistener;
    pub mod loxparser;
    pub mod loxvisitor;
}
mod errorvisitor;
mod interpvisitor;
mod trace;

fn main() {
    initlog();
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(PathBuf::from(args[1].clone()))
        .expect("Should have been able to read the file");

    let mut lexer = LoxLexer::new(InputStream::new(contents.as_str()));
    lexer.remove_error_listeners();
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LoxParser::new(token_source); //with_strategy(token_source, MyErrorStrategy::new());

    parser.remove_error_listeners();
    let el = Box::new(MyErrorListener::new());
    parser.add_error_listener(el);

    let root = parser.program();
    match root {
        Ok(root) => {
            let mut ed_visitor = ErrDetectVisit::new();
            root.accept(&mut ed_visitor);
            if ed_visitor.val != ErrVal::Empty {
                println!("Error: {:?}", ed_visitor.val);
                return;
            }

            let mut visitor = InterpVisit::new();
            root.accept(&mut visitor);
            println!("Result: {:?}", visitor.value());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
