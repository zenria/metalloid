use crate::state::ApplyStatus;
use crate::{State, Target};
use std::cell::RefCell;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
enum GraphError {
    #[error("Found cyclic dependency!")]
    CyclicDependency,
}

/// Inter-dependant states
struct Graph {
    // node id is the position in the nodes vector
    nodes: RefCell<Vec<Node>>,
    // key depends on list of values
    dependencies: RefCell<HashMap<NodeId, Vec<NodeDependency>>>,
}

struct NodeDependency {
    depends_on: NodeId,
    /// if present dependency will be applied only if this condition is true
    ///
    /// so if present and condition returns false, the dependant node will not be applied
    pre_condition: Box<dyn Fn(&dyn Target) -> bool>,
    //  if present the dependant state will be apply if the condition returns true
    apply_condition: Box<dyn Fn(ApplyStatus) -> bool>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: RefCell::new(vec![]),
            dependencies: RefCell::new(HashMap::new()),
        }
    }

    pub fn add<S: State + 'static>(&self, state: S) -> NodeRef {
        let mut nodes = self.nodes.borrow_mut();
        let node_id = nodes.len();
        nodes.push(Node {
            node_id,
            state: Box::new(state),
        });
        NodeRef {
            node_id,
            graph: &self,
        }
    }

    pub fn add_dependency<Pre, Cond>(
        &self,
        node: NodeId,
        depends_on: NodeId,
        pre_condition: Pre,
        apply_condition: Cond,
    ) where
        Pre: Fn(&dyn Target) -> bool + 'static,
        Cond: Fn(ApplyStatus) -> bool + 'static,
    {
        let mut dependencies = self.dependencies.borrow_mut();
        dependencies
            .entry(node)
            .or_insert(vec![])
            .push(NodeDependency {
                depends_on,
                pre_condition: Box::new(pre_condition),
                apply_condition: Box::new(apply_condition),
            });
    }
}

type NodeId = usize;

#[derive(Copy, Clone)]
struct NodeRef<'a> {
    node_id: NodeId,
    graph: &'a Graph,
}

struct Node {
    node_id: NodeId,
    state: Box<dyn State>,
}

impl<'a> NodeRef<'a> {
    fn depends_on(self, depends_on: NodeRef<'a>) -> Self {
        self.graph
            .add_dependency(self.node_id, depends_on.node_id, |_| true, |_| true);
        self
    }

    fn apply_if_changed(self, apply_if: NodeRef<'a>) -> Self {
        self.graph.add_dependency(
            self.node_id,
            apply_if.node_id,
            |_| true,
            |dependency_status| dependency_status == ApplyStatus::Changed,
        );
        self
    }
}
