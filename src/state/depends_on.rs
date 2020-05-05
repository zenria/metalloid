use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub trait DependsOn: State + Sized {
    fn depends_on(self, dep: &dyn State) -> DependOnState<Self> {
        DependOnState { state: self, dep }
    }
}
pub struct DependOnState<'dep, S: State> {
    state: S,
    dep: &'dep dyn State,
}
impl<'dep, S: State> State for DependOnState<'dep, S> {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        self.dep.apply(executor, target)?;
        self.state.apply(executor, target)
    }

    fn name(&self) -> String {
        self.state.name()
    }
}
