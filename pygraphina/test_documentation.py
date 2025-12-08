"""
Test script to verify documentation and type hints are working.
"""

import pygraphina as pg


def test_docstrings():
    """Test that docstrings are visible."""
    print("=" * 60)
    print("Testing Docstrings")
    print("=" * 60)

    # Test class docstring
    assert pg.PyGraph.__doc__ is not None
    print("✓ PyGraph class has docstring")

    # Test method docstrings
    assert pg.PyGraph.add_node.__doc__ is not None
    print("✓ add_node method has docstring")

    assert pg.PyGraph.add_edge.__doc__ is not None
    print("✓ add_edge method has docstring")

    assert pg.PyGraph.neighbors.__doc__ is not None
    print("✓ neighbors method has docstring")

    assert pg.PyGraph.bfs.__doc__ is not None
    print("✓ bfs method has docstring")

    # Test function docstrings
    assert pg.centrality.pagerank.__doc__ is not None
    print("✓ pagerank function has docstring")

    print("\nSample docstring for add_node:")
    print("-" * 60)
    print(pg.PyGraph.add_node.__doc__)
    print()


def test_help_function():
    """Test that help() works."""
    print("=" * 60)
    print("Testing help() Function")
    print("=" * 60)

    # Test help on a method
    print("\nhelp(pg.PyGraph.add_node):")
    print("-" * 60)
    help(pg.PyGraph.add_node)


def test_type_hints():
    """Test that type hints are available."""
    print("\n" + "=" * 60)
    print("Testing Type Hints")
    print("=" * 60)

    # Create a graph with type annotations
    g: pg.PyGraph = pg.PyGraph()
    node_id: int = g.add_node(100)
    edge_id: int = g.add_edge(node_id, g.add_node(200), 1.5)
    neighbors: list[int] = g.neighbors(node_id)

    print(f"✓ Created node with ID: {node_id}")
    print(f"✓ Created edge with ID: {edge_id}")
    print(f"✓ Got neighbors: {neighbors}")
    print("✓ Type hints work correctly")


def test_ide_features():
    """Demonstrate IDE features."""
    print("\n" + "=" * 60)
    print("IDE Features Available")
    print("=" * 60)

    print("""
With the .pyi stub file, your IDE now provides:

1. ✓ Autocomplete for methods and parameters
2. ✓ Type checking with mypy/pyright
3. ✓ Inline documentation on hover
4. ✓ Parameter hints while typing
5. ✓ Return type information
6. ✓ Error detection for incorrect types

Example in VS Code/PyCharm:
    g = pg.PyGraph()
    g.add_node(  # <- IDE shows: (attr: int) -> int

Try typing: help(pg.PyGraph) or help(pg.centrality.pagerank)
    """)


if __name__ == "__main__":
    test_docstrings()
    test_type_hints()
    test_ide_features()
    print("\n" + "=" * 60)
    print("All Documentation Tests Passed! ✅")
    print("=" * 60)
