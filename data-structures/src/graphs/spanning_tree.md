# Spanning Tree

A **spanning tree** of a graph is a subgraph that includes all the vertices of the original graph and is a single connected tree. In other words, it is a subset of the graph that covers all the vertices with the minimum possible number of edges, and without any cycles.

## Properties of Spanning Trees
- A spanning tree has \( V - 1 \) edges, where \( V \) is the number of vertices in the graph.
- There can be multiple spanning trees for a given graph.
- Removing one edge from a spanning tree will make it a disconnected graph.
- Adding one edge to a spanning tree will create a cycle.

## Example of a Spanning Tree

Consider the following graph:

```
    A
   / \
  B   C
 / \ / \
D   E   F
```

One possible spanning tree of this graph is:

```
    A
   / \
  B   C
 /     \
D       F
```

In this spanning tree:
- All vertices (A, B, C, D, F) are included.
- There are no cycles.
- The number of edges is \( V - 1 = 5 - 1 = 4 \).

Another possible spanning tree is:

```
    A
   / \
  B   C
   \   \
    E   F
```

In this spanning tree:
- All vertices (A, B, C, E, F) are included.
- There are no cycles.
- The number of edges is \( V - 1 = 5 - 1 = 4 \).
