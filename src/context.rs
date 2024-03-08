pub trait Logger {
    fn info(message: &str);

    fn warn(message: &str);

    fn debug(message: &str);
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Context {
    pub logger: &'static dyn Logger,
}