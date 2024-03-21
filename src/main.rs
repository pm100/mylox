use antlr::loxparser::{
    Assignment_altContext, Bool_falseContext, Bool_trueContext, ComparisonContext, EqualityContext,
    ExpressionContext, GroupContext, Logic_andContext, Logic_orContext, NilContext, NumberContext,
    StrvalContext, Unary_altContext,
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat, Tree},
    InputStream,
};
use std::collections::HashMap;
use std::unreachable;

mod antlr {
    pub mod loxlexer;
    pub mod loxlistener;
    pub mod loxparser;
    pub mod loxvisitor;
}
use crate::antlr::{
    loxlexer::LoxLexer,
    loxparser::{
        Assignment_altContextAttrs, ComparisonContextAttrs, EqualityContextAttrs,
        FactorContextAttrs, GroupContextAttrs, LoxParser, LoxParserContextType, TermContextAttrs,
        Unary_altContextAttrs, VarDeclContextAttrs,
    },
    loxvisitor::LoxVisitorCompat,
};
struct LoxVisit {
    val: TermValue,
    variables: HashMap<String, TermValue>,
}
impl LoxVisit {
    pub fn new() -> Self {
        Self {
            val: TermValue::Empty,
            variables: HashMap::new(),
        }
    }
}
impl ParseTreeVisitorCompat<'_> for LoxVisit {
    type Node = LoxParserContextType;
    type Return = TermValue;
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.val
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        //  unreachable!();
        next
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
enum TermValue {
    Number(f64),
    True,
    False,
    Nil,
    StringValue(String),
    #[default]
    Empty,
}
use antlr_rust::tree::Visitable;
impl LoxVisitorCompat<'_> for LoxVisit {
    ///type Return = AstNode;
    // fn visit_printstmt(&mut self, ctx: &antlr::loxparser::PrintstmtContext<'_>) -> TermValue {
    //     // println!("visit_printstmt");
    //     let res = ParseTreeVisitorCompat::visit_children(self, ctx);
    //     match res {
    //         TermValue::Number(x) => {
    //             println!("Number({})", x);
    //         }
    //         _ => {
    //             println!("Empty");
    //         }
    //     }
    //     res
    // }
    // fn visit_printStmt(&mut self, ctx: &antlr::loxparser::PrintStmtContext<'_>) -> TermValue {
    //     println!("visit_printStmt");
    //     println!("{:?}", ctx.exp);
    //     let res = ParseTreeVisitorCompat::visit_children(self, ctx);
    //     //  let res = self.visit(ctx.exp);
    //     println!("{:?}", res);
    //     res
    // }
    // fn visit_primary(&mut self, ctx: &antlr::loxparser::PrimaryContext<'_>) -> TermValue {
    //     println!("visit_primary");
    //      let x = ctx.get_child(0).unwrap();

    //     TermValue::Empty
    // }
    // fn visit_expression(&mut self, ctx: &antlr::loxparser::ExpressionContext<'_>) -> TermValue {}
    // fn visit_number(&mut self, ctx: &antlr::loxparser::NumberContext<'_>) -> TermValue {
    //     println!("visit_number");
    //     let text = ctx.get_text();
    //     TermValue::Number(text.parse().unwrap())
    // }
    // fn visit_term(&mut self, ctx: &antlr::loxparser::TermContext<'_>) -> TermValue {
    //     println!("visit_term");
    //     ctx.left.as_ref().unwrap().accept(self);
    //     if let Some(right) = &ctx.right {
    //         right.as_ref().accept(self);
    //     }
    //     // let right_node = self._nodes.pop().unwrap();
    //     // let left_node = self._nodes.pop().unwrap();
    //     // TermValue::Number(0.0)
    //     ParseTreeVisitorCompat::visit_children(self, ctx);
    // }
    // fn visit_factor(&mut self, ctx: &antlr::loxparser::FactorContext<'_>) -> Self::Return {
    //     println!("visit_factor");
    //     ctx
    //     TermValue::Number(0.0)
    // }

    // fn visit_pri(&mut self, ctx: &antlr::loxparser::PriContext<'_>) -> TermValue {

    // }

    fn visit_printStmt(&mut self, ctx: &antlr::loxparser::PrintStmtContext<'_>) -> Self::Return {
        println!("visit_printStmt");
        let res = self.visit(&*ctx.exp.as_ref().unwrap().as_ref());
        println!("{:?}", res);
        res
    }
    fn visit_varDecl(&mut self, ctx: &antlr::loxparser::VarDeclContext<'_>) -> Self::Return {
        println!("visit_varDecl");
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.expr.as_ref().unwrap().as_ref());
        self.variables.insert(id.clone(), val.clone());
        println!("{} = {:?}", id, val);
        TermValue::Empty
    }
    fn visit_identifier(&mut self, ctx: &antlr::loxparser::IdentifierContext<'_>) -> Self::Return {
        println!("visit_identifier");
        let id = ctx.get_text();
        if let Some(val) = self.variables.get(&id) {
            return val.clone();
        }
        panic!("Variable not found")
    }
    fn visit_expression(&mut self, ctx: &ExpressionContext) -> Self::Return {
        println!("visit_expression");
        self.visit(ctx.get_child(0).as_ref().unwrap().as_ref())
    }
    fn visit_logic_or_alt(
        &mut self,
        ctx: &antlr::loxparser::Logic_or_altContext<'_>,
    ) -> Self::Return {
        self.visit(ctx.get_child(0).as_ref().unwrap().as_ref())
    }
    fn visit_assignment_alt(&mut self, ctx: &Assignment_altContext) -> TermValue {
        println!("visit_asg");
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.iter.as_ref().unwrap().as_ref());
        self.variables.insert(id.clone(), val.clone());
        println!("{} = {:?}", id, val);
        TermValue::Empty
    }
    fn visit_logic_or(&mut self, ctx: &Logic_orContext) -> TermValue {
        println!("visit_logic_or");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if left == TermValue::True {
            return TermValue::True;
        }
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        right
    }
    fn visit_primary_alt(
        &mut self,
        ctx: &antlr::loxparser::Primary_altContext<'_>,
    ) -> Self::Return {
        self.visit(ctx.get_child(0).as_ref().unwrap().as_ref())
    }
    fn visit_logic_and(&mut self, ctx: &Logic_andContext) -> TermValue {
        println!("visit_logic_and");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if left == TermValue::False {
            return TermValue::False;
        }
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        right
    }
    fn visit_equality(&mut self, ctx: &EqualityContext) -> TermValue {
        println!("visit_equality");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if ctx.NEQ().is_some() {
            if left != right {
                return TermValue::True;
            } else {
                return TermValue::False;
            }
        } else if ctx.EQ().is_some() {
            if left == right {
                return TermValue::True;
            } else {
                return TermValue::False;
            }
        }
        panic!("x");
    }
    fn visit_comparison(&mut self, ctx: &ComparisonContext) -> TermValue {
        println!("visit_comparison");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if ctx.GT().is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return if x > y {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                }
                panic!("LL");
            }
            panic!("X")
        } else if ctx.GTE().is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return if x >= y {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                }
                panic!("LL");
            }
            panic!("X")
        } else if ctx.LT().is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return if x < y {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                }
                panic!("LL");
            }
            panic!("X")
        } else if ctx.LTE().is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return if x <= y {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                }
                panic!("LL");
            }
            panic!("X")
        }
        panic!("x");
    }
    fn visit_term(&mut self, ctx: &antlr::loxparser::TermContext<'_>) -> TermValue {
        println!("visit_term");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if ctx.PLUS(0).is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return TermValue::Number(x + y);
                }
                panic!("LL");
            }
            panic!("X")
        } else if ctx.MINUS(0).is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return TermValue::Number(x - y);
                }
                panic!("LL");
            }
            panic!("X")
        }
        panic!("x");
    }
    fn visit_factor(&mut self, ctx: &antlr::loxparser::FactorContext<'_>) -> TermValue {
        println!("visit_factor");

        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if ctx.STAR(0).is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return TermValue::Number(x * y);
                }
                panic!("LL");
            }
            panic!("X")
        } else if ctx.SLASH(0).is_some() {
            if let TermValue::Number(x) = left {
                if let TermValue::Number(y) = right {
                    return TermValue::Number(x / y);
                }
                panic!("LL");
            }
            panic!("X")
        }
        panic!("x");
    }
    fn visit_unary_alt(&mut self, ctx: &Unary_altContext) -> TermValue {
        println!("visit_unary");
        let right = self.visit(&*ctx.unary().unwrap());
        if ctx.BANG().is_some() {
            if right == TermValue::True {
                TermValue::False
            } else {
                TermValue::True
            }
        } else if ctx.MINUS().is_some() {
            if let TermValue::Number(x) = right {
                TermValue::Number(-x)
            } else {
                panic!("Expected number")
            }
        } else {
            right
        }
    }
    fn visit_group(&mut self, ctx: &GroupContext) -> Self::Return {
        println!("visit_group");
        let res = self.visit(ctx.expression().as_ref().unwrap().as_ref());
        res
    }
    fn visit_bool_false(&mut self, _ctx: &Bool_falseContext) -> TermValue {
        println!("visit_bool_false");
        TermValue::False
    }
    fn visit_bool_true(&mut self, _ctx: &Bool_trueContext) -> TermValue {
        println!("visit_bool_true");
        TermValue::True
    }
    fn visit_number(&mut self, ctx: &NumberContext) -> TermValue {
        println!("visit_number");
        let text = ctx.get_text();
        TermValue::Number(text.parse().unwrap())
    }
    fn visit_nil(&mut self, _ctx: &NilContext) -> TermValue {
        println!("visit_nil");
        TermValue::Nil
    }
    fn visit_strval(&mut self, ctx: &StrvalContext) -> TermValue {
        println!("visit_strval");
        TermValue::StringValue(ctx.get_text())
    }
}
fn main() {
    let mut _lexer = LoxLexer::new(InputStream::new("var a = 2+3; print a;".into()));
    let token_source = CommonTokenStream::new(_lexer);
    // token_source.iter().for_each(|token| {
    //     println!("{:?}", token);
    // });
    let mut parser = LoxParser::new(token_source);

    let root = parser.program().unwrap();
    println!("{:?}", root.to_string_tree(&*parser));

    let mut visitor = LoxVisit::new();
    root.accept(&mut visitor);
    //visitor.visit(&*root);
}
