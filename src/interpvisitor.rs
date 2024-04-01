use crate::{
    antlr::{
        self,
        loxparser::{
            Assignment_altContextAttrs, ComparisonContextAttrs, EqualityContextAttrs,
            FactorContextAttrs, FunctionDeclContextAttrs, GroupContextAttrs, IdentifierContext,
            LoxParserContextType, PrintStmtContext, TermContextAttrs, Unary_altContextAttrs,
            VarDeclContext, VarDeclContextAttrs,
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

use std::unreachable;
use std::{collections::HashMap, rc::Rc};
struct ExecutionState<'a> {
    pub variables: HashMap<String, TermValue>,
    pub return_value: TermValue,
    pub functions: Vec<Rc<antlr::loxparser::BlockContextAll<'a>>>,
}

impl<'a> ExecutionState<'a> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: Vec::new(),
            return_value: TermValue::Empty,
        }
    }
}
pub struct InterpVisit<'a> {
    // val: TermValue,
    state: Vec<ExecutionState<'a>>,
    break_requested: bool,
    loop_depth: u32,
}

impl<'a> InterpVisit<'a> {
    pub fn new() -> Self {
        Self {
            //val: TermValue::Empty,
            state: vec![ExecutionState::new()],
            break_requested: false,
            loop_depth: 0,
        }
    }
    pub fn value(&self) -> &TermValue {
        &self.state[self.state.len() - 1].return_value
    }
}
impl<'a> ParseTreeVisitorCompat<'a> for InterpVisit<'a> {
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
#[derive(Debug, Default, Clone)]
pub enum TermValue {
    Number(f64),
    True,
    False,
    Nil,
    StringValue(String),
    #[default]
    Empty,
    Error(String),
    Function(usize),
}
impl PartialEq for TermValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::True, Self::True) => true,
            (Self::False, Self::False) => true,
            (Self::Nil, Self::Nil) => true,
            (Self::StringValue(a), Self::StringValue(b)) => a == b,
            (Self::Empty, Self::Empty) => true,
            (Self::Error(a), Self::Error(b)) => a == b,
            (Self::Function(_), Self::Function(_)) => true,
            _ => false,
        }
    }
}
impl<'a> LoxVisitorCompat<'a> for InterpVisit<'a> {
    fn visit_program(&mut self, ctx: &antlr::loxparser::ProgramContext<'a>) -> Self::Return {
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
    fn visit_printStmt(&mut self, ctx: &PrintStmtContext<'a>) -> Self::Return {
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
    fn visit_varDecl(&mut self, ctx: &VarDeclContext<'a>) -> Self::Return {
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
    fn visit_functionDecl(
        &mut self,
        ctx: &antlr::loxparser::FunctionDeclContext<'a>,
    ) -> Self::Return {
        trace!("visit_functionDecl {:?}", ctx.get_text());
        let top = self.state.len() - 1;
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.body.as_ref().unwrap().as_ref());
        if let Some(ref b) = ctx.body {
            self.state[top].functions.push(b.clone());
        }
        // self.state[top].functions.push(*ctx.body.unwrap().clone());
        // if let TermValue::Error(_) = val {
        //     return val;
        // }
        let fidx = self.state[top].functions.len() - 1;
        self.state[top]
            .variables
            .insert(id.clone(), TermValue::Function(fidx));

        let f = self.state[top].functions[fidx].clone();

        let val = self.visit(f.as_ref());
        TermValue::Empty
    }
    fn visit_block(&mut self, ctx: &antlr::loxparser::BlockContext<'a>) -> Self::Return {
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
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'a>) -> Self::Return {
        trace!("visit_identifier {:?}", ctx.get_text());
        let id = ctx.get_text();
        for i in (0..self.state.len()).rev() {
            if let Some(val) = self.state[i].variables.get(&id) {
                return val.clone();
            }
        }
        // if let Some(val) = self.state[self.state.len() - 1].variables.get(&id) {
        //     return val.clone();
        // }

        TermValue::Error(format!("Variable {} not found", id))
    }
    fn visit_ifStmt(&mut self, ctx: &antlr::loxparser::IfStmtContext<'a>) -> Self::Return {
        trace!("visit_ifStmt {:?}", ctx.get_text());
        let cond = self.visit(&*ctx.condition.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = cond {
            return cond;
        }
        if cond == TermValue::True {
            return self.visit(&*ctx.thenBranch.as_ref().unwrap().as_ref());
        }
        if ctx.elseBranch.is_none() {
            return TermValue::Empty;
        }
        self.visit(&*ctx.elseBranch.as_ref().unwrap().as_ref())
    }
    fn visit_whileStmt(&mut self, ctx: &antlr::loxparser::WhileStmtContext<'a>) -> Self::Return {
        trace!("visit_whileStmt {:?}", ctx.get_text());
        let mut result = Self::Return::default();
        loop {
            self.loop_depth += 1;
            let cond = self.visit(&*ctx.condition.as_ref().unwrap().as_ref());
            match cond {
                TermValue::Error(_) => {
                    self.loop_depth -= 1;

                    return cond;
                }
                TermValue::True => {
                    result = self.visit(&*ctx.body.as_ref().unwrap().as_ref());
                    if self.break_requested {
                        self.break_requested = false;
                        self.loop_depth -= 1;

                        break;
                    }
                    if let TermValue::Error(_) = result {
                        self.loop_depth -= 1;

                        return result;
                    }
                }
                TermValue::False => {
                    self.loop_depth -= 1;

                    break;
                }
                _ => unreachable!(),
            }
        }
        result
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'a>) -> Self::Return {
        trace!("visit_expression {:?}", ctx.get_text());
        self.visit(ctx.get_child(0).as_ref().unwrap().as_ref())
    }
    fn visit_logic_or_alt(
        &mut self,
        ctx: &antlr::loxparser::Logic_or_altContext<'a>,
    ) -> Self::Return {
        self.visit(ctx.get_child(0).as_ref().unwrap().as_ref())
    }
    fn visit_forStmt(&mut self, ctx: &antlr::loxparser::ForStmtContext<'a>) -> Self::Return {
        trace!("visit_forStmt {:?}", ctx.get_text());
        let mut result = Self::Return::default();
        if ctx.forvar.is_some() {
            self.visit(&*ctx.forvar.as_ref().unwrap().as_ref());
        } else if ctx.initializer.is_some() {
            self.visit(&*ctx.initializer.as_ref().unwrap().as_ref());
        }
        loop {
            self.loop_depth += 1;
            // defer! {
            //     self.loop_depth -= 1;
            // }
            if let Some(cond) = ctx.condition.as_ref() {
                let cond = self.visit(&*cond.as_ref());
                match cond {
                    TermValue::Error(_) => {
                        self.loop_depth -= 1;
                        return cond;
                    }
                    TermValue::True => {}
                    TermValue::False => {
                        self.loop_depth -= 1;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            result = self.visit(&*ctx.body.as_ref().unwrap().as_ref());
            if self.break_requested {
                self.break_requested = false;
                self.loop_depth -= 1;

                break;
            }
            if let TermValue::Error(_) = result {
                self.loop_depth -= 1;
                return result;
            }
            if ctx.increment.is_some() {
                self.visit(&*ctx.increment.as_ref().unwrap().as_ref());
            }
        }

        result
    }
    fn visit_breakStmt(&mut self, ctx: &antlr::loxparser::BreakStmtContext<'a>) -> Self::Return {
        trace!("visit_breakStmt {:?}", ctx.get_text());
        if self.loop_depth == 0 {
            return TermValue::Error("break outside of loop".to_string());
        }
        self.break_requested = true;
        TermValue::Empty
    }
    // fn visit_assignment(&mut self, ctx: &antlr::loxparser::AssignmentContext<'_>) -> Self::Return {
    //     trace!("visit_assignment {:?}", ctx.get_text());
    //     let id = ctx.IDENTIFIER().unwrap().get_text();
    //     let val = self.visit(&*ctx.iter.as_ref().unwrap().as_ref());
    //     if let TermValue::Error(_) = val {
    //         return val;
    //     }
    //     let top = self.state.len() - 1;
    //     self.state[top].variables.insert(id.clone(), val.clone());
    //     val
    // }

    fn visit_assignment_alt(&mut self, ctx: &Assignment_altContext<'a>) -> TermValue {
        trace!("visit_assignment_alt {:?}", ctx.get_text());
        let id = ctx.IDENTIFIER().unwrap().get_text();
        let val = self.visit(&*ctx.iter.as_ref().unwrap().as_ref());
        if let TermValue::Error(_) = val {
            return val;
        }

        for i in (0..self.state.len()).rev() {
            if let Some(old_val) = self.state[i].variables.get_mut(&id) {
                *old_val = val.clone();
                return val;
            }
        }
        return TermValue::Error(format!("Variable {} not found", id));
    }
    fn visit_logic_or(&mut self, ctx: &Logic_orContext<'a>) -> TermValue {
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
    // fn visit_(&mut self, ctx: &antlr::loxparser::Primary_altContext<'a>) -> Self::Return {
    //     self.visit(ctx.get_child(0).as_ref().unwrap().as_ref())
    // }

    fn visit_callfun(&mut self, ctx: &antlr::loxparser::CallfunContext<'a>) -> Self::Return {
        trace!("visit_callfun {:?}", ctx.get_text());
        let id = ctx.id.as_ref().unwrap().get_text();
        let top = self.state.len() - 1;
        let fidx = match self.state[top].variables.get(&id) {
            Some(TermValue::Function(fidx)) => *fidx,
            _ => {
                return TermValue::Error(format!("Function {} not found", id));
            }
        };
        let f = self.state[top].functions[fidx].clone();
        let mut result = Self::Return::default();
        self.state.push(ExecutionState::new());
        for node in f.get_children() {
            result = self.visit(node.as_ref());
            if let TermValue::Error(_) = result {
                return result;
            }
        }
        self.state.pop();
        result
    }
    fn visit_logic_and(&mut self, ctx: &Logic_andContext<'a>) -> TermValue {
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
    fn visit_equality(&mut self, ctx: &EqualityContext<'a>) -> TermValue {
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
    fn visit_comparison(&mut self, ctx: &ComparisonContext<'a>) -> TermValue {
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
    fn visit_term(&mut self, ctx: &antlr::loxparser::TermContext<'a>) -> TermValue {
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
    fn visit_factor(&mut self, ctx: &antlr::loxparser::FactorContext<'a>) -> TermValue {
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
    fn visit_unary_alt(&mut self, ctx: &Unary_altContext<'a>) -> TermValue {
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
    fn visit_group(&mut self, ctx: &GroupContext<'a>) -> Self::Return {
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
