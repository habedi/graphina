// File: tests/graph_exceptions_tests.rs

use graphina::core::exceptions::*;
use std::error::Error;

#[test]
fn test_graphina_exception() {
    let err = GraphinaException::new("base exception");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaException"));
    assert!(s.contains("base exception"));
    // Ensure it implements the Error trait.
    let _e: &dyn Error = &err;
}

#[test]
fn test_graphina_error() {
    let err = GraphinaError::new("serious error");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaError"));
    assert!(s.contains("serious error"));
}

#[test]
fn test_pointless_concept() {
    let err = GraphinaPointlessConcept::new("null graph provided");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaPointlessConcept"));
    assert!(s.contains("null graph provided"));
}

#[test]
fn test_algorithm_error() {
    let err = GraphinaAlgorithmError::new("unexpected termination");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaAlgorithmError"));
    assert!(s.contains("unexpected termination"));
}

#[test]
fn test_unfeasible() {
    let err = GraphinaUnfeasible::new("no feasible solution");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaUnfeasible"));
    assert!(s.contains("no feasible solution"));
}

#[test]
fn test_no_path() {
    let err = GraphinaNoPath::new("no path exists");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaNoPath"));
    assert!(s.contains("no path exists"));
}

#[test]
fn test_no_cycle() {
    let err = GraphinaNoCycle::new("no cycle exists");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaNoCycle"));
    assert!(s.contains("no cycle exists"));
}

#[test]
fn test_node_not_found() {
    let err = NodeNotFound::new("node 42 not found");
    let s = format!("{}", err);
    assert!(s.contains("NodeNotFound"));
    assert!(s.contains("node 42 not found"));
}

#[test]
fn test_has_a_cycle() {
    let err = HasACycle::new("graph has a cycle");
    let s = format!("{}", err);
    assert!(s.contains("HasACycle"));
    assert!(s.contains("graph has a cycle"));
}

#[test]
fn test_unbounded() {
    let err = GraphinaUnbounded::new("optimization unbounded");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaUnbounded"));
    assert!(s.contains("optimization unbounded"));
}

#[test]
fn test_not_implemented() {
    let err = GraphinaNotImplemented::new("feature not implemented");
    let s = format!("{}", err);
    assert!(s.contains("GraphinaNotImplemented"));
    assert!(s.contains("feature not implemented"));
}

#[test]
fn test_ambiguous_solution() {
    let err = AmbiguousSolution::new("multiple solutions exist");
    let s = format!("{}", err);
    assert!(s.contains("AmbiguousSolution"));
    assert!(s.contains("multiple solutions exist"));
}

#[test]
fn test_exceeded_max_iterations() {
    let err = ExceededMaxIterations::new("iteration limit reached");
    let s = format!("{}", err);
    assert!(s.contains("ExceededMaxIterations"));
    assert!(s.contains("iteration limit reached"));
}

#[test]
fn test_power_iteration_failed_convergence() {
    let err = PowerIterationFailedConvergence::new(50, "failed to converge");
    let s = format!("{}", err);
    assert!(s.contains("PowerIterationFailedConvergence"));
    assert!(s.contains("50"));
    assert!(s.contains("failed to converge"));
}
