pub trait Context {
    fn enter(&mut self) {}
    fn exit(&mut self) {}
}

#[macro_export]
macro_rules! with {
    ($ctx:expr, $blk:block) => {
        let ctx = &mut $ctx;
        crate::context::Context::enter(ctx);
        $blk;
        crate::context::Context::exit(ctx);
    }
}
