use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub struct IfChangedState<'dep, S: State, D: State> {
    state: S,
    dep: &'dep D,
}
impl<'dep, S: State, D: State> IfChangedState<'dep, S, D> {
    pub(crate) fn new(state: S, dep: &'dep D) -> Self {
        Self { state, dep }
    }
}

impl<'dep, S: State, D: State> State for IfChangedState<'dep, S, D> {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        if self.dep.apply(executor, target)? == ApplyStatus::Changed {
            self.state.apply(executor, target)
        } else {
            Ok(ApplyStatus::NotChanged)
        }
    }

    fn name(&self) -> String {
        self.state.name()
    }
}
