use crate::{
    antlr::{
        self,
        loxparser::{
            Assignment_altContextAttrs, ComparisonContextAttrs, EqualityContextAttrs,
            FactorContextAttrs, GroupContextAttrs, IdentifierContext, LoxParserContextType,
            PrintStmtContext, TermContextAttrs, Unary_altContextAttrs, VarDeclContext,
            VarDeclContextAttrs,
        },
        loxvisitor::LoxVisitorCompat,
    },
    trace,
};
use antlr::loxparser::{
    Assignment_altContext, Bool_falseContext, Bool_trueContext, ComparisonContext, EqualityContext,
    ExpressionContext, GroupContext, Logic_andContext, Logic_orContext, NilContext, NumberContext,
    StrvalContext, Unary_altContext,
};
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat, Tree};

use std::collections::HashMap;
use std::unreachable;

struct ExecutionState {
    pub variables: HashMap<String, TermValue>,
    pub return_value: TermValue,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            return_value: TermValue::Empty,
        }
    }
}
pub struct InterpVisit {
    // val: TermValue,
    state: Vec<ExecutionState>,
}

impl InterpVisit {
    pub fn new() -> Self {
        Self {
            //val: TermValue::Empty,
            state: vec![ExecutionState::new()],
        }
    }
}
impl ParseTreeVisitorCompat<'_> for InterpVisit {
    type Node = LoxParserContextType;
    type Return = TermValue;
    fn temp_result(&mut self) -> &mut Self::Return {
        let top = self.state.len() - 1;
        &mut self.state[top].return_value
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
        trace!("visit_program");

        let mut result = Self::Return::default();
        for node in ctx.get_children() {
            result = self.visit(node.as_ref());

            if let TermValue::Error(_) = result {
                return result;
            }
        }
        return result;
    }
    fn visit_printStmt(&mut self, ctx: &PrintStmtContext) -> Self::Return {
        trace!("visit_printStmt {:?}", ctx.get_text());
        let res = self.visit(&*ctx.exp.as_ref().unwrap().as_ref());
        match &res {
            TermValue::Error(_) => {
                return res;
            }
            TermValue::Nil => {
                println!("nil");
            }
            TermValue::True => {
                println!("true");
            }
            TermValue::False => {
                println!("false");
            }
            TermValue::Number(x) => {
                println!("{}", x);
            }
            TermValue::StringValue(x) => {
                println!("{}", x);
            }
            _ => {
                println!("unknown");
            }
        }
        res
    }
    fn visit_varDecl(&mut self, ctx: &VarDeclContext) -> Self::Return {
        trace!("visit_varDecl {:?}", ctx.get_text());
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.expr.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = val {
            return val;
        }
        let top = self.state.len() - 1;
        self.state[top].variables.insert(id.clone(), val.clone());
        TermValue::Empty
    }
    fn visit_block(&mut self, ctx: &antlr::loxparser::BlockContext<'_>) -> Self::Return {
        trace!("visit_block {:?}", ctx.get_text());
        let mut result = Self::Return::default();
        self.state.push(ExecutionState::new());
        for node in ctx.get_children() {
            trace!("visit_block child {:?}", node);

            result = self.visit(node.as_ref());
            if let TermValue::Error(_) = result {
                return result;
            }
        }
        self.state.pop();
        result
    }
    fn visit_identifier(&mut self, ctx: &IdentifierContext) -> Self::Return {
        trace!("visit_identifier {:?}", ctx.get_text());
        let id = ctx.get_text();
        if let Some(val) = self.state[self.state.len() - 1].variables.get(&id) {
            return val.clone();
        }
        TermValue::Error("Variable not found".to_string())
    }
    fn visit_expression(&mut self, ctx: &ExpressionContext) -> Self::Return {
        trace!("visit_expression {:?}", ctx.get_text());
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
        trace!("visit_assignment_alt {:?}", ctx.get_text());
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.iter.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = val {
            return val;
        }
        let top = self.state.len() - 1;
        self.state[top].variables.insert(id.clone(), val.clone());
        val
    }
    fn visit_logic_or(&mut self, ctx: &Logic_orContext) -> TermValue {
        trace!("visit_logic_or {:?}", ctx.get_text());
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
        trace!("visit_logic_and {:?}", ctx.get_text());
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
        trace!("visit_equality {:?}", ctx.get_text());
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
        trace!("visit_comparison {:?}", ctx.get_text());
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
        trace!("visit_term {:?}", ctx.get_text());
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
        trace!("visit_factor {:?}", ctx.get_text());
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
        trace!("visit_unary_alt {:?}", ctx.get_text());
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
        trace!("visit_group {:?}", ctx.get_text());
        let res = self.visit(ctx.expression().as_ref().unwrap().as_ref());
        res
    }
    fn visit_bool_false(&mut self, _ctx: &Bool_falseContext) -> TermValue {
        trace!("visit_bool_false");
        TermValue::False
    }
    fn visit_bool_true(&mut self, _ctx: &Bool_trueContext) -> TermValue {
        trace!("visit_bool_true");
        TermValue::True
    }
    fn visit_number(&mut self, ctx: &NumberContext) -> TermValue {
        let text = ctx.get_text();
        trace!("visit_number {:?}", text);
        TermValue::Number(text.parse().unwrap())
    }
    fn visit_nil(&mut self, _ctx: &NilContext) -> TermValue {
        trace!("visit_nil");
        TermValue::Nil
    }
    fn visit_strval(&mut self, ctx: &StrvalContext) -> TermValue {
        trace!("visit_strval {:?}", ctx.get_text());
        let str = ctx.get_text();
        let str = str.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        TermValue::StringValue(str.to_string())
    }
}
