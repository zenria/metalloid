use crate::executor::NOOPExecutor;
use crate::state::{ApplyError, ApplyStatus, PrintAndApplyRandomly, NOOP};
use crate::{Executor, State, Target};
use daggy::{Dag, NodeIndex, Walker};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use thiserror::Error;

#[derive(Error, Debug)]
enum GraphError {
    #[error("Found cyclic dependency!")]
    CyclicDependency,
}

/// Inter-dependant states
struct Graph {
    // node id is the position in the nodes vector
    inner: RefCell<Dag<Node, NodeDependency>>,

    independent_nodes: RefCell<BTreeSet<NodeId>>,

    name: String,
}

struct NodeDependency {
    /// if present dependency will be applied only if this condition is true
    ///
    /// so if present and condition returns false, the dependant node will not be applied
    pre_condition: Box<dyn Fn(&dyn Target) -> bool>,
    //  if present the dependant state will be apply if the condition returns true
    apply_condition: Box<dyn Fn(ApplyStatus) -> bool>,
}

impl Graph {
    pub fn new(name: &str) -> Self {
        Self {
            inner: RefCell::new(Default::default()),
            independent_nodes: RefCell::new(Default::default()),
            name: name.into(),
        }
    }

    pub fn add<S: State + 'static>(&self, state: S) -> NodeRef {
        let mut graph = self.inner.borrow_mut();
        let node_id = graph.add_node(Node {
            state: Box::new(state),
        });
        self.independent_nodes.borrow_mut().insert(node_id);
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
    ) -> Result<(), GraphError>
    where
        Pre: Fn(&dyn Target) -> bool + 'static,
        Cond: Fn(ApplyStatus) -> bool + 'static,
    {
        self.independent_nodes.borrow_mut().remove(&node);
        let mut graph = self.inner.borrow_mut();
        graph
            .add_edge(
                node,
                depends_on,
                NodeDependency {
                    pre_condition: Box::new(pre_condition),
                    apply_condition: Box::new(apply_condition),
                },
            )
            .map_err(|_e| GraphError::CyclicDependency)
            .map(|_edge| ())
    }

    fn apply_on_node(
        &self,
        node_id: NodeId,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        let graph = self.inner.borrow();
        // apply the node
        let mut apply_result = graph
            .node_weight(node_id)
            .unwrap()
            .state
            .apply(executor, target)?;
        // is this a dependency ?
        let parents = graph.parents(node_id).iter(&graph);
        for (edge_id, parent_node_id) in parents {
            let dependency = graph.edge_weight(edge_id).unwrap();
            if (dependency.apply_condition)(apply_result) {
                apply_result =
                    apply_result + self.apply_on_node(parent_node_id, executor, target)?;
            }
        }
        Ok(apply_result)
    }
}

impl State for Graph {
    fn apply(
        &self,
        executor: &dyn Executor,
        target: &dyn Target,
    ) -> Result<ApplyStatus, ApplyError> {
        self.independent_nodes
            .borrow()
            .iter()
            .try_fold(ApplyStatus::NotChanged, |ret, node_id| {
                Ok(ret + self.apply_on_node(*node_id, executor, target)?)
            })
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

type NodeId = NodeIndex<u32>;

#[derive(Copy, Clone)]
struct NodeRef<'a> {
    node_id: NodeId,
    graph: &'a Graph,
}

struct Node {
    state: Box<dyn State>,
}

impl<'a> NodeRef<'a> {
    fn depends_on(self, depends_on: NodeRef<'a>) -> Result<Self, GraphError> {
        self.graph
            .add_dependency(self.node_id, depends_on.node_id, |_| true, |_| true)
            .map(|_| self)
    }

    fn apply_if_changed(self, apply_if: NodeRef<'a>) -> Result<Self, GraphError> {
        self.graph
            .add_dependency(
                self.node_id,
                apply_if.node_id,
                |_| true,
                |dependency_status| dependency_status == ApplyStatus::Changed,
            )
            .map(|_| self)
    }
}
#[cfg(test)]
#[test]
fn test() {
    let g = Graph::new("install me");
    let me_package = g.add(PrintAndApplyRandomly("install me package"));
    let me_service = g.add(PrintAndApplyRandomly("install me service"));
    let me_user = g.add(PrintAndApplyRandomly("create me user"));
    let me_group = g.add(PrintAndApplyRandomly("create me group"));

    me_service.depends_on(me_package).unwrap();
    me_service.depends_on(me_user).unwrap();
    me_service.depends_on(me_group).unwrap();

    me_package.depends_on(me_user).unwrap();
    me_user.depends_on(me_group).unwrap();

    g.apply(&NOOPExecutor, &crate::tests::TestTarget).unwrap();
}
