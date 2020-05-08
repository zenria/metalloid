use crate::{Executor, Target};

use crate::state::compose::ComposedState;
use crate::state::only_if::OnlyIfState;
use std::ops::Add;
use thiserror::Error;

pub(crate) mod compose;
pub(crate) mod only_if;

#[derive(Error, Debug)]
#[error("Error executing {0}")]
pub struct ApplyError(String);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

pub trait State {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError>;

    fn name(&self) -> String;
}

impl<T: State + ?Sized> StateExt for T {}

pub trait StateExt: State {
    fn compose<R>(self, other: R) -> ComposedState<Self, R>
    where
        R: State,
        Self: Sized,
    {
        ComposedState::new(self, other)
    }

    fn only_if<F>(self, cond: F) -> OnlyIfState<Self, F>
    where
        F: Fn(&dyn Target) -> bool,
        Self: Sized,
    {
        OnlyIfState::new(self, cond)
    }
}

pub struct NOOP(pub &'static str);
impl State for NOOP {
    fn apply(
        &self,
        _executor: &dyn Executor,
        _target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        Ok(ApplyStatus::NotChanged)
    }

    fn name(&self) -> String {
        self.0.into()
    }
}

pub struct PrintAndApplyRandomly(pub &'static str);
impl State for PrintAndApplyRandomly {
    fn apply(
        &self,
        _executor: &dyn Executor,
        _target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        let state = if rand::random() {
            ApplyStatus::Changed
        } else {
            ApplyStatus::NotChanged
        };
        println!("{} - {:?}", self.0, state);
        Ok(state)
    }

    fn name(&self) -> String {
        self.0.into()
    }
}
