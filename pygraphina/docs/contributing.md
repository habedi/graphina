# Contributing to PyGraphina

Thank you for your interest in contributing to PyGraphina! This guide will help you get started.

## Ways to Contribute

There are many ways to contribute to PyGraphina:

- **Report bugs**: Found a bug? [Open an issue](https://github.com/habedi/graphina/issues)
- **Suggest features**: Have an idea? Share it in the issues
- **Improve documentation**: Fix typos, add examples, clarify explanations
- **Write code**: Fix bugs, add features, optimize algorithms
- **Add tests**: Improve test coverage
- **Share your work**: Write blog posts, create tutorials, give talks

## Getting Started

### 1. Set Up Development Environment

First, fork and clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/graphina.git
cd graphina/pygraphina
```

Install development dependencies:

```bash
# Create a virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -e ".[dev]"

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the package
pip install maturin[zig]
maturin develop --release
```

### 2. Create a Branch

Create a branch for your changes:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

Use descriptive branch names:

- `feature/add-clustering-algorithm`
- `fix/pagerank-convergence-bug`
- `docs/improve-quickstart-guide`

### 3. Make Your Changes

Follow the coding standards (see below) and make your changes.

### 4. Test Your Changes

Run the test suite:

```bash
# Run all tests
pytest

# Run with coverage
pytest --cov=pygraphina --cov-report=term

# Run specific test file
pytest pygraphina/tests/test_centrality.py
```

### 5. Commit Your Changes

Write clear commit messages:

```bash
git add .
git commit -m "Add: Support for weighted Katz centrality"
```

Commit message format:

- `Add: New feature or functionality`
- `Fix: Bug fix`
- `Docs: Documentation changes`
- `Test: Add or modify tests`
- `Refactor: Code restructuring without behavior change`
- `Perf: Performance improvements`

### 6. Push and Create a Pull Request

```bash
git push origin feature/your-feature-name
```

Then go to GitHub and create a pull request.

## Coding Standards

### Python Code

PyGraphina uses:

- **Ruff** for linting and formatting
- **mypy** for type checking

Run before committing:

```bash
# Format code
ruff format .

# Check for issues
ruff check .

# Type checking
mypy pygraphina/
```

Python style guidelines:

- Follow PEP 8
- Use type hints
- Write docstrings (Google style)
- Keep functions focused and small
- Use meaningful variable names

Example:

```python
def calculate_centrality(
    graph: PyGraph,
    method: str = "pagerank",
    **kwargs: Any
) -> Dict[int, float]:
    """Calculate node centrality scores.

    Args:
        graph: The graph to analyze.
        method: The centrality method to use.
        **kwargs: Additional algorithm-specific parameters.

    Returns:
        Dictionary mapping node IDs to centrality scores.

    Raises:
        ValueError: If method is not recognized.
    """
    if method == "pagerank":
        return pagerank(graph, **kwargs)
    elif method == "betweenness":
        return betweenness(graph)
    else:
        raise ValueError(f"Unknown method: {method}")
```

### Rust Code

For Rust code in PyGraphina:

- Follow Rust conventions
- Use `clippy` for linting: `cargo clippy`
- Format with `rustfmt`: `cargo fmt`
- Add doc comments for public APIs

Example:

```rust
/// Calculate PageRank centrality scores.
///
/// # Arguments
/// * `graph` - The graph to analyze
/// * `damping` - Damping factor (0.0 to 1.0)
/// * `max_iters` - Maximum iterations
/// * `tol` - Convergence tolerance
///
/// # Returns
/// Dictionary mapping node IDs to PageRank scores
#[pyfunction]
pub fn pagerank(
    graph: &PyGraph,
    damping: f64,
    max_iters: usize,
    tol: f64,
) -> PyResult<HashMap<usize, f64>> {
    // Implementation
}
```

## Testing

### Writing Tests

Place tests in `pygraphina/tests/`:

```python
import pytest
import pygraphina as pg


def test_pagerank_simple():
    """Test PageRank on a simple graph."""
    g = pg.PyGraph()
    a, b, c = [g.add_node(i) for i in range(3)]
    g.add_edge(a, b, 1.0)
    g.add_edge(b, c, 1.0)

    scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)

    assert len(scores) == 3
    assert all(0 <= score <= 1 for score in scores.values())
    assert abs(sum(scores.values()) - 1.0) < 1e-6


def test_pagerank_empty_graph():
    """Test PageRank on empty graph."""
    g = pg.PyGraph()
    scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
    assert scores == {}


@pytest.mark.parametrize("damping", [0.5, 0.85, 0.95])
def test_pagerank_damping_factor(damping):
    """Test PageRank with different damping factors."""
    g = pg.PyGraph()
    nodes = [g.add_node(i) for i in range(5)]
    for i in range(4):
        g.add_edge(nodes[i], nodes[i + 1], 1.0)

    scores = pg.centrality.pagerank(g, damping, 100, 1e-6)
    assert len(scores) == 5
```

### Test Coverage

Aim for high test coverage:

- Unit tests for individual functions
- Integration tests for algorithm combinations
- Edge cases (empty graphs, single nodes, etc.)
- Error handling tests

Check coverage:

```bash
pytest --cov=pygraphina --cov-report=html
# Open htmlcov/index.html in browser
```

## Documentation

### Docstrings

Use Google-style docstrings:

```python
def my_function(param1: int, param2: str) -> bool:
    """Short one-line description.

    Longer description if needed. Can span multiple lines
    and include more details about the function.

    Args:
        param1: Description of param1.
        param2: Description of param2.

    Returns:
        Description of return value.

    Raises:
        ValueError: When something is wrong.

    Example:
        >>> result = my_function(42, "hello")
        >>> print(result)
        True
    """
    return True
```

### Documentation Pages

When adding new features, update or add documentation in `docs/`:

- API reference pages in `docs/api/`
- Examples in `docs/examples/`
- Guides in `docs/getting-started/`

Use clear headings, code examples, and explanations.

## Pull Request Guidelines

### Before Submitting

- [ ] Code passes all tests
- [ ] Code is formatted with ruff
- [ ] Type checking passes (mypy)
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Commit messages are clear

### PR Description

Include in your PR description:

1. **What**: What does this PR do?
2. **Why**: Why is this change needed?
3. **How**: How does it work?
4. **Testing**: How was it tested?
5. **Breaking changes**: Any breaking changes?

Example:

```markdown
## Add Weighted Katz Centrality

### What
Adds support for weighted edge values in Katz centrality calculation.

### Why
The current implementation ignores edge weights. This is needed for weighted graphs.

### How
Modified the Katz algorithm to multiply the adjacency matrix by edge weights.

### Testing
- Added unit tests for weighted graphs
- Verified results match NetworkX for same inputs
- Tested with various weight distributions

### Breaking Changes
None. This is backward compatible.
```

## Community Guidelines

### Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Provide constructive feedback
- Focus on the code, not the person

### Communication

- **GitHub Issues**: Bug reports, feature requests
- **Pull Requests**: Code contributions
- **Discussions**: General questions, ideas

### Getting Help

If you need help:

1. Check existing documentation
2. Search closed issues
3. Open a new issue with a clear description
4. Provide minimal reproducible examples

## Recognition

Contributors are recognized in:

- GitHub contributors page
- Release notes
- Documentation acknowledgments

Thank you for contributing to PyGraphina!

## Quick Links

- [GitHub Repository](https://github.com/habedi/graphina)
- [Issue Tracker](https://github.com/habedi/graphina/issues)
- [Documentation](index.md)
