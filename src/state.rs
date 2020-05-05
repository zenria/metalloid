use crate::{Executor, Target};

use thiserror::Error;

pub(crate) mod compose;
pub(crate) mod condition;
pub(crate) mod depends_on;
pub(crate) mod graph;
pub(crate) mod if_changed;

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

pub trait State {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError>;

    fn name(&self) -> String;
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
