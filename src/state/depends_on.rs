use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub struct DependOnState<'dep, S: State, D: State> {
    state: S,
    dep: &'dep D,
}
impl<'dep, S: State, D: State> DependOnState<'dep, S, D> {
    pub(crate) fn new(state: S, dep: &'dep D) -> Self {
        Self { state, dep }
    }
}

impl<'dep, S: State, D: State> State for DependOnState<'dep, S, D> {
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
