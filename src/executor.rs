use futures::future::{/*ExecuteError, Executor,*/ Future, FutureExt};
use std::sync::Arc;
use std::pin::Pin;
use std::ops::Deref;

/*
trait Executor<F: Future<Output = Result<(), ()>>> {
    fn execute(&self, future: Pin<Box<F>>) -> Result<(), ExecuteError<F>>;
}

struct ExecuteError<F> {
    f: F,
}
*/
pub trait Executor: /*std::fmt::Debug +*/ Send + Sync {
    fn spawn(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) -> Result<(), ()>;
}


/*
pub trait PulsarExecutor: Executor<BoxSendFuture> + Send + Sync + 'static {}

impl<T: Executor<BoxSendFuture> + Send + Sync + 'static> PulsarExecutor for T {}

type BoxSendFuture = Box<dyn Future<Output = Result<(), ()>> + Send + 'static>;
*/

#[derive(Clone)]
pub struct TaskExecutor {
    inner: Arc<dyn Executor + Send + Sync + 'static>,
}

impl TaskExecutor {
    pub fn new<E>(exec: E) -> Self
    where
        E: Executor + 'static,
    {
        Self {
            inner: Arc::new(exec),
        }
    }

    fn execute<F>(&self, f: F) -> Result<(), ()>
        where
            F: Future<Output = Result<(), ()>> + Send + 'static,
    {
        match self.inner.spawn(Box::pin(f.map(|_| ()))) {
            Ok(()) => Ok(()),
            Err(_) => panic!("no executor available"),
        }
    }
    /*pub fn spawn<F>(&self, f: F)
    where
        F: Future<Output = Result<(), ()>> + Send + 'static,
    {
        if self.execute(f).is_err() {
            panic!("no executor available")
        }
    }*/
}

impl Executor for TaskExecutor
// where
//    F: Future<Output = Result<(), ()>> + Send + 'static,
{
    fn spawn(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) -> Result<(), ()> {
        self.inner.deref().spawn(f)
    }
}
