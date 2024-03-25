/// `ANTLRError::FallThrough` Error returned `BailErrorStrategy` to bail out from parsing
#[derive(Debug)]
pub struct ParseCancelledError(ANTLRError);

impl Error for ParseCancelledError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl Display for ParseCancelledError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ParseCancelledError, caused by ")?;
        self.0.fmt(f)
    }
}

impl<'a, T: Parser<'a>> ErrorStrategy<'a, T> for MyErrorStrategy<'a, T::Node> {
    #[inline(always)]
    fn reset(&mut self, recognizer: &mut T) {
        self.0.reset(recognizer)
    }

    #[cold]
    fn recover_inline(
        &mut self,
        recognizer: &mut T,
    ) -> Result<<T::TF as TokenFactory<'a>>::Tok, ANTLRError> {
        let err = ANTLRError::InputMismatchError(InputMisMatchError::new(recognizer));

        self.report_error(recognizer, &err);
        Err(err)
    }

    #[cold]
    fn recover(&mut self, recognizer: &mut T, e: &ANTLRError) -> Result<(), ANTLRError> {
        Err(self.process_error(recognizer, &e))
    }

    #[inline(always)]
    fn sync(&mut self, _recognizer: &mut T) -> Result<(), ANTLRError> {
        /* empty */
        Ok(())
    }

    #[inline(always)]
    fn in_error_recovery_mode(&mut self, recognizer: &mut T) -> bool {
        self.0.in_error_recovery_mode(recognizer)
    }

    #[inline(always)]
    fn report_error(&mut self, recognizer: &mut T, e: &ANTLRError) {
        self.0.report_error(recognizer, e)
    }

    #[inline(always)]
    fn report_match(&mut self, _recognizer: &mut T) {}
}
//use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
// struct MyErrorStrategy;
// better_any::tid! {impl<'i,Ctx> TidAble<'i> for MyErrorStrategy where Ctx:ParserNodeType<'i> }
// impl<'a, T: Parser<'a>> ErrorStrategy<'a, T> for MyErrorStrategy {
//     fn reset(&mut self, recognizer: &mut T) {
//         todo!()
//     }
//     fn recover_inline(&mut self, _recognizer: &T) {
//         println!("recover_inline");
//     }

//     fn recover(
//         &mut self,
//         recognizer: &mut T,
//         e: &antlr_rust::errors::ANTLRError,
//     ) -> Result<(), antlr_rust::errors::ANTLRError> {
//         todo!()
//     }

//     fn sync(&mut self, _recognizer: &mut T) {
//         println!("sync");
//     }

//     fn in_error_recovery_mode(&mut self, recognizer: &mut T) -> bool {
//         todo!()
//     }

//     fn report_error(&mut self, recognizer: &mut T, e: &antlr_rust::errors::ANTLRError) {
//         todo!()
//     }

//     fn report_match(&mut self, recognizer: &mut T) {
//         todo!()
//     }
// }


#[derive(Default, Debug)]
pub struct MyErrorStrategy<'input, Ctx: ParserNodeType<'input>>(DefaultErrorStrategy<'input, Ctx>);

better_any::tid! {impl<'i,Ctx> TidAble<'i> for MyErrorStrategy<'i,Ctx> where Ctx:ParserNodeType<'i> }

impl<'input, Ctx: ParserNodeType<'input>> MyErrorStrategy<'input, Ctx> {
    /// Creates new instance of `BailErrorStrategy`
    pub fn new() -> Self {
        Self(DefaultErrorStrategy::new())
    }

    fn process_error<T: Parser<'input, Node = Ctx, TF = Ctx::TF>>(
        &self,
        recognizer: &mut T,
        e: &ANTLRError,
    ) -> ANTLRError {
        let mut ctx = recognizer.get_parser_rule_context().clone();
        // let _: Option<()> = (|| {
        //     loop {
        //         ctx.set_exception(e.clone());
        //         ctx = ctx.get_parent()?
        //     }
        //     Some(())
        // })();
        return ANTLRError::FallThrough(Rc::new(ParseCancelledError(e.clone())));
    }
}
