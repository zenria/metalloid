pub trait Executor {
    // TODO idea here is to have local executor, remote (ssh or grpc with some control&command servers)
}

pub struct NOOPExecutor;
impl Executor for NOOPExecutor {}
