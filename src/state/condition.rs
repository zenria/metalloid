use crate::state::{ApplyError, ApplyStatus};
use crate::Executor;
use crate::State;
use crate::Target;

pub struct CondState {
    not_applied: Option<Box<dyn State>>,
    conditions: Vec<(Box<dyn State>, Box<dyn Fn(&dyn Target) -> bool>)>,
}

impl CondState {
    pub fn new() -> Self {
        Self {
            not_applied: None,
            conditions: vec![],
        }
    }

    /// Apply given state if condition is true
    pub fn apply_if<F: Fn(&dyn Target) -> bool + 'static, S: State + 'static>(
        mut self,
        state: S,
        cond: F,
    ) -> Self {
        self.conditions.push((Box::new(state), Box::new(cond)));
        self
    }

    /// Apply only if nothing has been applied with apply_if, can be used as fallback or to fail the state
    pub fn not_applied<S: State + 'static>(mut self, state: S) -> Self {
        self.not_applied = Some(Box::new(state));
        self
    }
}

impl State for CondState {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        if let Some(status) = self
            .conditions
            .iter()
            .filter(|(_state, cond)| cond(target as &dyn Target))
            .try_fold(None, |status, (state, _)| {
                Ok(Some(
                    state.apply(executor, target)? + status.unwrap_or(ApplyStatus::NotChanged),
                ))
            })?
        {
            Ok(status)
        } else {
            if let Some(not_applied_state) = &self.not_applied {
                not_applied_state.apply(executor, target)
            } else {
                Ok(ApplyStatus::NotChanged)
            }
        }
    }

    fn name(&self) -> String {
        let mut ret = String::new();
        for (state, _cond) in &self.conditions {
            ret.push_str(&state.name());
            ret.push_str(" / ");
        }
        if let Some(not_applied) = &self.not_applied {
            ret.push_str(" - ");
            ret.push_str(&not_applied.name());
        }
        ret
    }
}
