/*!
# Unified Error Type

This module provides a unified error enum that consolidates all Graphina error types
for better ergonomics and error handling consistency.
*/

use std::error::Error;
use std::fmt;

/// Unified error type for all Graphina operations.
///
/// This enum consolidates all error types for better error handling and pattern matching.
#[derive(Debug)]
pub enum GraphinaError {
    /// General-purpose error
    Generic(String),

    /// Node not found in graph
    NodeNotFound(String),

    /// Edge not found in graph
    EdgeNotFound(String),

    /// No path exists between nodes
    NoPath(String),

    /// No cycle exists in graph
    NoCycle(String),

    /// Graph has a cycle when acyclic structure is expected
    HasCycle(String),

    /// Graph is empty or invalid for the operation
    InvalidGraph(String),

    /// Algorithm terminated unexpectedly
    AlgorithmError(String),

    /// No feasible solution exists
    Unfeasible(String),

    /// Optimization problem is unbounded
    Unbounded(String),

    /// Feature not implemented for this graph type
    NotImplemented(String),

    /// Ambiguous solution exists
    AmbiguousSolution(String),

    /// Exceeded maximum iterations
    ExceededMaxIterations { iterations: usize, message: String },

    /// Power iteration failed to converge
    ConvergenceFailed { iterations: usize, message: String },

    /// I/O error
    IoError(String),

    /// Serialization/deserialization error
    SerializationError(String),

    /// Invalid argument or parameter
    InvalidArgument(String),
}

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
}

impl fmt::Display for GraphinaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GraphinaError::Generic(msg) => write!(f, "Graphina error: {}", msg),
            GraphinaError::NodeNotFound(msg) => write!(f, "Node not found: {}", msg),
            GraphinaError::EdgeNotFound(msg) => write!(f, "Edge not found: {}", msg),
            GraphinaError::NoPath(msg) => write!(f, "No path exists: {}", msg),
            GraphinaError::NoCycle(msg) => write!(f, "No cycle exists: {}", msg),
            GraphinaError::HasCycle(msg) => write!(f, "Graph has a cycle: {}", msg),
            GraphinaError::InvalidGraph(msg) => write!(f, "Invalid graph: {}", msg),
            GraphinaError::AlgorithmError(msg) => write!(f, "Algorithm error: {}", msg),
            GraphinaError::Unfeasible(msg) => write!(f, "No feasible solution: {}", msg),
            GraphinaError::Unbounded(msg) => write!(f, "Unbounded solution: {}", msg),
            GraphinaError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            GraphinaError::AmbiguousSolution(msg) => write!(f, "Ambiguous solution: {}", msg),
            GraphinaError::ExceededMaxIterations {
                iterations,
                message,
            } => {
                write!(f, "Exceeded max iterations ({}): {}", iterations, message)
            }
            GraphinaError::ConvergenceFailed {
                iterations,
                message,
            } => {
                write!(
                    f,
                    "Convergence failed after {} iterations: {}",
                    iterations, message
                )
            }
            GraphinaError::IoError(msg) => write!(f, "I/O error: {}", msg),
            GraphinaError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            GraphinaError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl Error for GraphinaError {}

// Implement From conversions for backward compatibility
impl From<crate::core::exceptions::GraphinaException> for GraphinaError {
    fn from(e: crate::core::exceptions::GraphinaException) -> Self {
        GraphinaError::Generic(e.message)
    }
}

impl From<crate::core::exceptions::NodeNotFound> for GraphinaError {
    fn from(e: crate::core::exceptions::NodeNotFound) -> Self {
        GraphinaError::NodeNotFound(e.message)
    }
}

impl From<crate::core::exceptions::GraphinaNoPath> for GraphinaError {
    fn from(e: crate::core::exceptions::GraphinaNoPath) -> Self {
        GraphinaError::NoPath(e.message)
    }
}

impl From<crate::core::exceptions::PowerIterationFailedConvergence> for GraphinaError {
    fn from(e: crate::core::exceptions::PowerIterationFailedConvergence) -> Self {
        GraphinaError::ConvergenceFailed {
            iterations: e.num_iterations,
            message: e.message,
        }
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
        assert_eq!(format!("{}", err), "Graphina error: test error");

        let err = GraphinaError::node_not_found("node 5");
        assert_eq!(format!("{}", err), "Node not found: node 5");

        let err = GraphinaError::convergence_failed(100, "did not converge");
        assert_eq!(
            format!("{}", err),
            "Convergence failed after 100 iterations: did not converge"
        );
    }

    #[test]
    fn test_error_conversions() {
        let old_err = crate::core::exceptions::GraphinaException::new("test");
        let new_err: GraphinaError = old_err.into();
        assert!(matches!(new_err, GraphinaError::Generic(_)));
    }
}
