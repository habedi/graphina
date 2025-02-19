// File: src/core/exceptions.rs

/*!
# Graphina Exceptions

This module defines the custom error types for Graphina. These exceptions are used
throughout the library to indicate various failure conditions and provide detailed
error information. Each exception implements the standard [`Error`](std::error::Error)
and [`Display`](std::fmt::Display) traits.

## Usage

Create an exception using the `new` method and inspect it via its display implementation:

```rust
use graphina::core::exceptions::GraphinaException;
let err = GraphinaException::new("A generic error occurred.");
println!("{}", err); // Prints: GraphinaException: A generic error occurred.
```
*/

use std::error::Error;
use std::fmt;

/// Base exception for Graphina.
///
/// This error type serves as a general exception that can be used for non-specific error cases.
#[derive(Debug)]
pub struct GraphinaException {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaException {
    /// Creates a new `GraphinaException` with the specified message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use graphina::core::exceptions::GraphinaException;
    /// let err = GraphinaException::new("Something went wrong.");
    /// assert_eq!(format!("{}", err), "GraphinaException: Something went wrong.");
    /// ```
    pub fn new(message: &str) -> Self {
        GraphinaException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaException: {}", self.message)
    }
}

impl Error for GraphinaException {}

/// Exception for serious errors.
///
/// This error is intended for critical issues that may require immediate attention.
#[derive(Debug)]
pub struct GraphinaError {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaError {
    /// Creates a new `GraphinaError` with the given message.
    pub fn new(message: &str) -> Self {
        GraphinaError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaError: {}", self.message)
    }
}

impl Error for GraphinaError {}

/// Exception raised when a null graph is provided to an algorithm that cannot use it.
///
/// This error indicates that an algorithm received an invalid (null) graph input.
#[derive(Debug)]
pub struct GraphinaPointlessConcept {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaPointlessConcept {
    /// Creates a new `GraphinaPointlessConcept` error.
    pub fn new(message: &str) -> Self {
        GraphinaPointlessConcept {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaPointlessConcept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaPointlessConcept: {}", self.message)
    }
}

impl Error for GraphinaPointlessConcept {}

/// Exception for unexpected termination of algorithms.
///
/// This error is used when an algorithm terminates unexpectedly.
#[derive(Debug)]
pub struct GraphinaAlgorithmError {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaAlgorithmError {
    /// Creates a new `GraphinaAlgorithmError` with the provided message.
    pub fn new(message: &str) -> Self {
        GraphinaAlgorithmError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaAlgorithmError: {}", self.message)
    }
}

impl Error for GraphinaAlgorithmError {}

/// Exception raised when no feasible solution exists.
///
/// This error indicates that an algorithm failed to find a viable solution.
#[derive(Debug)]
pub struct GraphinaUnfeasible {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaUnfeasible {
    /// Creates a new `GraphinaUnfeasible` error.
    pub fn new(message: &str) -> Self {
        GraphinaUnfeasible {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaUnfeasible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaUnfeasible: {}", self.message)
    }
}

impl Error for GraphinaUnfeasible {}

/// Exception raised when no path exists between nodes.
///
/// This error is returned when an algorithm determines that no valid path can be found.
#[derive(Debug)]
pub struct GraphinaNoPath {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaNoPath {
    /// Creates a new `GraphinaNoPath` error.
    pub fn new(message: &str) -> Self {
        GraphinaNoPath {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaNoPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaNoPath: {}", self.message)
    }
}

impl Error for GraphinaNoPath {}

/// Exception raised when no cycle exists in a graph.
///
/// This error is used when an algorithm expects a cycle but none is found.
#[derive(Debug)]
pub struct GraphinaNoCycle {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaNoCycle {
    /// Creates a new `GraphinaNoCycle` error.
    pub fn new(message: &str) -> Self {
        GraphinaNoCycle {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaNoCycle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaNoCycle: {}", self.message)
    }
}

impl Error for GraphinaNoCycle {}

/// Exception raised if a requested node is not found.
///
/// This error is typically returned when an operation attempts to reference a non-existent node.
#[derive(Debug)]
pub struct NodeNotFound {
    /// Detailed error message.
    pub message: String,
}

impl NodeNotFound {
    /// Creates a new `NodeNotFound` error.
    pub fn new(message: &str) -> Self {
        NodeNotFound {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for NodeNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeNotFound: {}", self.message)
    }
}

impl Error for NodeNotFound {}

/// Exception raised if a graph has a cycle when an acyclic structure is expected.
///
/// This error indicates that a cycle was found in a graph where it should not exist.
#[derive(Debug)]
pub struct HasACycle {
    /// Detailed error message.
    pub message: String,
}

impl HasACycle {
    /// Creates a new `HasACycle` error.
    pub fn new(message: &str) -> Self {
        HasACycle {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for HasACycle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HasACycle: {}", self.message)
    }
}

impl Error for HasACycle {}

/// Exception raised when an optimization problem is unbounded.
///
/// This error is used when an algorithm detects that the solution is unbounded.
#[derive(Debug)]
pub struct GraphinaUnbounded {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaUnbounded {
    /// Creates a new `GraphinaUnbounded` error.
    pub fn new(message: &str) -> Self {
        GraphinaUnbounded {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaUnbounded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaUnbounded: {}", self.message)
    }
}

impl Error for GraphinaUnbounded {}

/// Exception raised for unimplemented algorithms for a given graph type.
///
/// This error indicates that a requested algorithm or feature is not yet available.
#[derive(Debug)]
pub struct GraphinaNotImplemented {
    /// Detailed error message.
    pub message: String,
}

impl GraphinaNotImplemented {
    /// Creates a new `GraphinaNotImplemented` error.
    pub fn new(message: &str) -> Self {
        GraphinaNotImplemented {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GraphinaNotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphinaNotImplemented: {}", self.message)
    }
}

impl Error for GraphinaNotImplemented {}

/// Raised when more than one valid solution exists for an intermediary step.
///
/// This error is used when an algorithm encounters ambiguity during a computation.
#[derive(Debug)]
pub struct AmbiguousSolution {
    /// Detailed error message.
    pub message: String,
}

impl AmbiguousSolution {
    /// Creates a new `AmbiguousSolution` error.
    pub fn new(message: &str) -> Self {
        AmbiguousSolution {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AmbiguousSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AmbiguousSolution: {}", self.message)
    }
}

impl Error for AmbiguousSolution {}

/// Raised if a loop iterates too many times without convergence.
///
/// This error signals that an iterative algorithm has exceeded the allowed iteration limit.
#[derive(Debug)]
pub struct ExceededMaxIterations {
    /// Detailed error message.
    pub message: String,
}

impl ExceededMaxIterations {
    /// Creates a new `ExceededMaxIterations` error.
    pub fn new(message: &str) -> Self {
        ExceededMaxIterations {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ExceededMaxIterations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExceededMaxIterations: {}", self.message)
    }
}

impl Error for ExceededMaxIterations {}

/// Raised when the power iteration method fails to converge within the iteration limit.
///
/// This error includes the number of iterations attempted before failure.
#[derive(Debug)]
pub struct PowerIterationFailedConvergence {
    /// The number of iterations performed.
    pub num_iterations: usize,
    /// Detailed error message.
    pub message: String,
}

impl PowerIterationFailedConvergence {
    /// Creates a new `PowerIterationFailedConvergence` error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use graphina::core::exceptions::PowerIterationFailedConvergence;
    /// let err = PowerIterationFailedConvergence::new(100, "Convergence not reached.");
    /// assert_eq!(format!("{}", err), "PowerIterationFailedConvergence after 100 iterations: Convergence not reached.");
    /// ```
    pub fn new(num_iterations: usize, message: &str) -> Self {
        PowerIterationFailedConvergence {
            num_iterations,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PowerIterationFailedConvergence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PowerIterationFailedConvergence after {} iterations: {}",
            self.num_iterations, self.message
        )
    }
}

impl Error for PowerIterationFailedConvergence {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphina_exception_display() {
        let err = GraphinaException::new("Generic error");
        assert_eq!(format!("{}", err), "GraphinaException: Generic error");
    }

    #[test]
    fn test_graphina_error_display() {
        let err = GraphinaError::new("Serious error");
        assert_eq!(format!("{}", err), "GraphinaError: Serious error");
    }

    #[test]
    fn test_pointless_concept_display() {
        let err = GraphinaPointlessConcept::new("Null graph provided");
        assert_eq!(
            format!("{}", err),
            "GraphinaPointlessConcept: Null graph provided"
        );
    }

    #[test]
    fn test_algorithm_error_display() {
        let err = GraphinaAlgorithmError::new("Unexpected termination");
        assert_eq!(
            format!("{}", err),
            "GraphinaAlgorithmError: Unexpected termination"
        );
    }

    #[test]
    fn test_unfeasible_display() {
        let err = GraphinaUnfeasible::new("No feasible solution");
        assert_eq!(
            format!("{}", err),
            "GraphinaUnfeasible: No feasible solution"
        );
    }

    #[test]
    fn test_no_path_display() {
        let err = GraphinaNoPath::new("No path exists");
        assert_eq!(format!("{}", err), "GraphinaNoPath: No path exists");
    }

    #[test]
    fn test_no_cycle_display() {
        let err = GraphinaNoCycle::new("No cycle found");
        assert_eq!(format!("{}", err), "GraphinaNoCycle: No cycle found");
    }

    #[test]
    fn test_node_not_found_display() {
        let err = NodeNotFound::new("Node missing");
        assert_eq!(format!("{}", err), "NodeNotFound: Node missing");
    }

    #[test]
    fn test_has_a_cycle_display() {
        let err = HasACycle::new("Cycle detected");
        assert_eq!(format!("{}", err), "HasACycle: Cycle detected");
    }

    #[test]
    fn test_unbounded_display() {
        let err = GraphinaUnbounded::new("Optimization unbounded");
        assert_eq!(
            format!("{}", err),
            "GraphinaUnbounded: Optimization unbounded"
        );
    }

    #[test]
    fn test_not_implemented_display() {
        let err = GraphinaNotImplemented::new("Feature not available");
        assert_eq!(
            format!("{}", err),
            "GraphinaNotImplemented: Feature not available"
        );
    }

    #[test]
    fn test_ambiguous_solution_display() {
        let err = AmbiguousSolution::new("Multiple solutions exist");
        assert_eq!(
            format!("{}", err),
            "AmbiguousSolution: Multiple solutions exist"
        );
    }

    #[test]
    fn test_exceeded_max_iterations_display() {
        let err = ExceededMaxIterations::new("Iteration limit exceeded");
        assert_eq!(
            format!("{}", err),
            "ExceededMaxIterations: Iteration limit exceeded"
        );
    }

    #[test]
    fn test_power_iteration_failed_convergence_display() {
        let err = PowerIterationFailedConvergence::new(150, "Failed to converge");
        assert_eq!(
            format!("{}", err),
            "PowerIterationFailedConvergence after 150 iterations: Failed to converge"
        );
    }
}
