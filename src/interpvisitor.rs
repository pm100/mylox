use crate::{
    antlr::{
        self,
        loxlexer::LoxLexer,
        loxparser::{
            Assignment_altContextAttrs, ComparisonContextAttrs, EqualityContextAttrs,
            FactorContextAttrs, GroupContextAttrs, IdentifierContext, LoxParser,
            LoxParserContextType, PrintStmtContext, TermContextAttrs, Unary_altContextAttrs,
            VarDeclContext, VarDeclContextAttrs,
        },
        loxvisitor::LoxVisitorCompat,
    },
    MyErrorListener,
};
use antlr::loxparser::{
    Assignment_altContext, Bool_falseContext, Bool_trueContext, ComparisonContext, EqualityContext,
    ExpressionContext, GroupContext, Logic_andContext, Logic_orContext, NilContext, NumberContext,
    StrvalContext, Unary_altContext,
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    error_listener::ErrorListener,
    parser_rule_context::ParserRuleContext,
    tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat, Tree},
    InputStream,
};
use antlr_rust::{tree::LeafNode, BaseParser};
use anyhow::Result;
use std::unreachable;
use std::{collections::HashMap, rc::Rc};

use antlr_rust::recognizer::Recognizer;
pub struct InterpVisit {
    val: TermValue,
    variables: HashMap<String, TermValue>,
    //el: &'a Box<dyn ErrorDector<'a, R>>,
}

impl InterpVisit {
    pub fn new() -> Self {
        Self {
            val: TermValue::Empty,
            variables: HashMap::new(),
        }
    }
}
impl ParseTreeVisitorCompat<'_> for InterpVisit {
    type Node = LoxParserContextType;
    type Return = TermValue;
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.val
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        //  unreachable!();
        next
    }
    fn visit_error_node(&mut self, node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        println!("visit_error_node");
        TermValue::Error(node.get_text())
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub enum TermValue {
    Number(f64),
    True,
    False,
    Nil,
    StringValue(String),
    #[default]
    Empty,
    Error(String),
}

impl LoxVisitorCompat<'_> for InterpVisit {
    fn visit_program(&mut self, ctx: &antlr::loxparser::ProgramContext<'_>) -> Self::Return {
        println!("visit_program");

        let mut result = Self::Return::default();
        for node in ctx.get_children() {
            let n = node.as_ref();
            println!("child: {:?}", n);
            println!("child: {:?}", node.get_text());
            result = self.visit(node.as_ref());

            if let TermValue::Error(_) = result {
                return result;
            }
        }
        return result;
    }
    fn visit_printStmt(&mut self, ctx: &PrintStmtContext) -> Self::Return {
        println!("visit_printStmt");
        let res = self.visit(&*ctx.exp.as_ref().unwrap().as_ref());
        println!("{:?}", res);
        res
    }
    fn visit_varDecl(&mut self, ctx: &VarDeclContext) -> Self::Return {
        println!("visit_varDecl");
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.expr.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = val {
            return val;
        }
        self.variables.insert(id.clone(), val.clone());
        println!("{} = {:?}", id, val);
        TermValue::Empty
    }
    fn visit_identifier(&mut self, ctx: &IdentifierContext) -> Self::Return {
        println!("visit_identifier");
        let id = ctx.get_text();
        if let Some(val) = self.variables.get(&id) {
            return val.clone();
        }
        TermValue::Error("Variable not found".to_string())
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
        if let TermValue::Error(_) = val {
            return val;
        }

        self.variables.insert(id.clone(), val.clone());
        println!("{} = {:?}", id, val);
        val
    }
    fn visit_logic_or(&mut self, ctx: &Logic_orContext) -> TermValue {
        println!("visit_logic_or");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = left {
            return left;
        }
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
        if let TermValue::Error(_) = left {
            return left;
        }
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
        if let TermValue::Error(_) = left {
            return left;
        }
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = right {
            return right;
        }
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
        unreachable!("impossible eq");
    }
    fn visit_comparison(&mut self, ctx: &ComparisonContext) -> TermValue {
        println!("visit_comparison");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = left {
            return left;
        }
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());

        if let TermValue::Error(_) = right {
            return right;
        }
        if let TermValue::Number(l) = left {
            if let TermValue::Number(r) = right {
                if ctx.GT().is_some() {
                    return if l > r {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                } else if ctx.GTE().is_some() {
                    return if l >= r {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                } else if ctx.LT().is_some() {
                    return if l < r {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                } else if ctx.LTE().is_some() {
                    return if l <= r {
                        TermValue::True
                    } else {
                        TermValue::False
                    };
                }
                unreachable!("impossible comparison");
            }
        }
        TermValue::Error("must both be numbers".to_string())
    }
    fn visit_term(&mut self, ctx: &antlr::loxparser::TermContext<'_>) -> TermValue {
        println!("visit_term");
        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = left {
            return left;
        }

        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = right {
            return right;
        }
        if let TermValue::Error(_) = right {
            return right;
        }
        if let TermValue::Number(l) = left {
            if let TermValue::Number(r) = right {
                if ctx.PLUS(0).is_some() {
                    return TermValue::Number(l + r);
                } else if ctx.MINUS(0).is_some() {
                    return TermValue::Number(l - r);
                }
            }
        }
        panic!("x");
    }
    fn visit_factor(&mut self, ctx: &antlr::loxparser::FactorContext<'_>) -> TermValue {
        println!("visit_factor");

        let left = self.visit(&*ctx.left.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = left {
            return left;
        }
        if ctx.right.is_none() {
            return left;
        }
        let right = self.visit(&*ctx.right.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = right {
            return right;
        }
        if let TermValue::Number(l) = left {
            if let TermValue::Number(r) = right {
                if ctx.STAR(0).is_some() {
                    return TermValue::Number(l * r);
                } else if ctx.SLASH(0).is_some() {
                    return TermValue::Number(l / r);
                }
            }
        }
        panic!("must be numbers");
    }
    fn visit_unary_alt(&mut self, ctx: &Unary_altContext) -> TermValue {
        println!("visit_unary");
        let right = self.visit(&*ctx.unary().unwrap());
        if let TermValue::Error(_) = right {
            return right;
        }
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
        let str = ctx.get_text();
        let str = str.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        TermValue::StringValue(str.to_string())
    }
}
