/*!
# Unified Error Type

This module provides a unified error enum using `thiserror` that consolidates all Graphina error types
for better ergonomics and error handling consistency.
*/

use thiserror::Error;

/// Unified error type for all Graphina operations.
///
/// This enum consolidates all error types for better error handling and pattern matching.
/// Uses `thiserror` for automatic `Display` and `Error` trait implementations.
#[derive(Error, Debug, Clone)]
pub enum GraphinaError {
    /// General-purpose error
    #[error("Graphina error: {0}")]
    Generic(String),

    /// Node not found in graph
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    /// Edge not found in graph
    #[error("Edge not found: {0}")]
    EdgeNotFound(String),

    /// No path exists between nodes
    #[error("No path exists: {0}")]
    NoPath(String),

    /// No cycle exists in graph
    #[error("No cycle exists: {0}")]
    NoCycle(String),

    /// Graph has a cycle when acyclic structure is expected
    #[error("Graph has a cycle: {0}")]
    HasCycle(String),

    /// Graph is empty or invalid for the operation
    #[error("Invalid graph: {0}")]
    InvalidGraph(String),

    /// Algorithm terminated unexpectedly
    #[error("Algorithm error: {0}")]
    AlgorithmError(String),

    /// No feasible solution exists
    #[error("No feasible solution: {0}")]
    Unfeasible(String),

    /// Optimization problem is unbounded
    #[error("Unbounded solution: {0}")]
    Unbounded(String),

    /// Feature not implemented for this graph type
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Ambiguous solution exists
    #[error("Ambiguous solution: {0}")]
    AmbiguousSolution(String),

    /// Exceeded maximum iterations
    #[error("Exceeded max iterations ({iterations}): {message}")]
    ExceededMaxIterations { iterations: usize, message: String },

    /// Power iteration failed to converge
    #[error("Convergence failed after {iterations} iterations: {message}")]
    ConvergenceFailed { iterations: usize, message: String },

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Invalid argument or parameter
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Graph operation on pointless/degenerate graph
    #[error("Pointless concept: {0}")]
    PointlessConcept(String),
}

/// Result type alias for Graphina operations
pub type Result<T> = std::result::Result<T, GraphinaError>;

impl GraphinaError {
    /// Creates a generic error with the given message.
    pub fn generic(message: impl Into<String>) -> Self {
        GraphinaError::Generic(message.into())
    }

    /// Creates a node not found error.
    pub fn node_not_found(message: impl Into<String>) -> Self {
        GraphinaError::NodeNotFound(message.into())
    }

    /// Creates an edge not found error.
    pub fn edge_not_found(message: impl Into<String>) -> Self {
        GraphinaError::EdgeNotFound(message.into())
    }

    /// Creates a no path error.
    pub fn no_path(message: impl Into<String>) -> Self {
        GraphinaError::NoPath(message.into())
    }

    /// Creates an invalid graph error.
    pub fn invalid_graph(message: impl Into<String>) -> Self {
        GraphinaError::InvalidGraph(message.into())
    }

    /// Creates an algorithm error.
    pub fn algorithm_error(message: impl Into<String>) -> Self {
        GraphinaError::AlgorithmError(message.into())
    }

    /// Creates a convergence failed error.
    pub fn convergence_failed(iterations: usize, message: impl Into<String>) -> Self {
        GraphinaError::ConvergenceFailed {
            iterations,
            message: message.into(),
        }
    }

    /// Creates an invalid argument error.
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        GraphinaError::InvalidArgument(message.into())
    }

    /// Creates a not implemented error.
    pub fn not_implemented(message: impl Into<String>) -> Self {
        GraphinaError::NotImplemented(message.into())
    }

    /// Creates a has cycle error.
    pub fn has_cycle(message: impl Into<String>) -> Self {
        GraphinaError::HasCycle(message.into())
    }

    /// Creates a no cycle error.
    pub fn no_cycle(message: impl Into<String>) -> Self {
        GraphinaError::NoCycle(message.into())
    }

    /// Creates an unfeasible error.
    pub fn unfeasible(message: impl Into<String>) -> Self {
        GraphinaError::Unfeasible(message.into())
    }

    /// Creates a pointless concept error.
    pub fn pointless_concept(message: impl Into<String>) -> Self {
        GraphinaError::PointlessConcept(message.into())
    }
}

impl From<std::io::Error> for GraphinaError {
    fn from(e: std::io::Error) -> Self {
        GraphinaError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for GraphinaError {
    fn from(e: serde_json::Error) -> Self {
        GraphinaError::SerializationError(e.to_string())
    }
}

impl From<bincode::error::EncodeError> for GraphinaError {
    fn from(e: bincode::error::EncodeError) -> Self {
        GraphinaError::SerializationError(e.to_string())
    }
}

impl From<bincode::error::DecodeError> for GraphinaError {
    fn from(e: bincode::error::DecodeError) -> Self {
        GraphinaError::SerializationError(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = GraphinaError::generic("test error");
        assert_eq!(err.to_string(), "Graphina error: test error");
    }

    #[test]
    fn test_node_not_found() {
        let err = GraphinaError::node_not_found("Node 5");
        assert_eq!(err.to_string(), "Node not found: Node 5");
    }

    #[test]
    fn test_convergence_failed() {
        let err = GraphinaError::convergence_failed(100, "tolerance not met");
        assert!(err.to_string().contains("100 iterations"));
        assert!(err.to_string().contains("tolerance not met"));
    }

    #[test]
    fn test_error_is_clonable() {
        let err = GraphinaError::generic("test");
        let cloned = err.clone();
        assert_eq!(err.to_string(), cloned.to_string());
    }

    #[test]
    fn test_result_alias() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }
        assert_eq!(returns_result().unwrap(), 42);
    }
}
