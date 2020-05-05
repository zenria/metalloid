use crate::state::{ApplyError, ApplyStatus};
use crate::{Executor, State, Target};

/// A bunch of dependent States
///
pub struct GraphState<'dep> {
    name: String,
    states: Vec<GraphState<'dep>>,
    root: Option<Box<dyn State>>,
    dependencies: Vec<&'dep dyn State>,
    if_changed: Vec<&'dep dyn State>,
}

impl<'dep> GraphState<'dep> {
    pub fn new<F: FnOnce() -> Vec<GraphState<'dep>>>(name: &'static str, builder: F) -> Self {
        Self {
            name: name.into(),
            states: builder(),
            root: None,
            dependencies: vec![],
            if_changed: vec![],
        }
    }

    pub fn wrapping<S: State + 'static>(state: S) -> Self {
        Self {
            name: state.name(),
            states: vec![],
            root: Some(Box::new(state)),
            dependencies: vec![],
            if_changed: vec![],
        }
    }

    pub fn depends_on(&mut self, state: &'dep dyn State) -> &Self {
        self.dependencies.push(state);
        self
    }

    pub fn if_changed(&mut self, state: &'dep dyn State) -> &Self {
        self.if_changed.push(state);
        self
    }
}

pub struct GraphStateBuilder<'dep> {
    states: Vec<GraphState<'dep>>,
}

impl<'dep> GraphStateBuilder<'dep> {
    pub fn add<S: State + 'static>(&mut self, state: S) -> &'dep GraphState {
        self.states.push(GraphState::wrapping(state));
        self.states.get(self.states.len() - 1).unwrap()
    }
}

impl<'dep> State for GraphState<'dep> {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        // Apply any dependant states
        for depencency in &self.dependencies {
            depencency.apply(executor, target)?;
        }

        let apply_all_states = self
            .if_changed
            .iter()
            .try_fold(None, |status, state| {
                Ok(Some(
                    state.apply(executor, target)? + status.unwrap_or(ApplyStatus::NotChanged),
                ))
            })?
            .map(|status| status == ApplyStatus::Changed)
            .unwrap_or(true);
        let mut ret = ApplyStatus::NotChanged;

        if apply_all_states {
            if let Some(state) = self.root.as_ref() {
                ret = ret + state.apply(executor, target)?;
            }
            for state in &self.states {
                ret = ret + state.apply(executor, target)?;
            }
        }

        Ok(ret)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
