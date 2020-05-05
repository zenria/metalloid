use crate::{Executor, Target};

use crate::state::compose::ComposedState;
use crate::state::depends_on::DependOnState;
use crate::state::if_changed::IfChangedState;
use crate::state::only_if::OnlyIfState;
use thiserror::Error;

pub(crate) mod compose;
pub(crate) mod depends_on;
pub(crate) mod if_changed;
pub(crate) mod only_if;

#[derive(Error, Debug)]
#[error("Error executing {0}")]
pub struct ApplyError(String);

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ApplyStatus {
    Changed,
    NotChanged,
}

impl std::ops::Add for ApplyStatus {
    type Output = ApplyStatus;

    fn add(self, rhs: Self) -> Self::Output {
        if self == ApplyStatus::Changed || rhs == ApplyStatus::Changed {
            ApplyStatus::Changed
        } else {
            ApplyStatus::NotChanged
        }
    }
}

pub trait State: Sized {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError>;

    fn name(&self) -> String;

    fn depends_on<D: State>(self, dep: &D) -> DependOnState<Self, D> {
        DependOnState::new(self, dep)
    }

    fn if_changed<D: State>(self, dep: &D) -> IfChangedState<Self, D> {
        IfChangedState::new(self, dep)
    }

    fn compose<R: State>(self, other: R) -> ComposedState<Self, R> {
        ComposedState::new(self, other)
    }

    fn only_if<F: Fn(&dyn Target) -> bool>(self, cond: F) -> OnlyIfState<Self, F> {
        OnlyIfState::new(self, cond)
    }
}

pub struct NOOP;
impl State for NOOP {
    fn apply(
        &self,
        _executor: &dyn Executor,
        _target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        Ok(ApplyStatus::NotChanged)
    }

    fn name(&self) -> String {
        "no-operation".into()
    }
}
