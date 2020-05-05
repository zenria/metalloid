use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

pub trait IfChanged: State + Sized {
    fn if_changed(self, dep: &dyn State) -> IfChangedState<Self> {
        IfChangedState { state: self, dep }
    }
}
pub struct IfChangedState<'dep, S: State> {
    state: S,
    dep: &'dep dyn State,
}
impl<'dep, S: State> State for IfChangedState<'dep, S> {
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
