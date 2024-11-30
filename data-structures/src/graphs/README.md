# Graphs 

A graph is a non-linear data structure made up of nodes or vertices and edges. The nodes known as vertices are connected by edges. 


```plaintext
      A
     /|\
    / | \
   B--C--D
    \ | /
     \|/
      E
```

In this example, we have a **graph** with 5 nodes labeled **A, B, C, D, and E**. The graph is fully connected, meaning there is an edge between every pair of nodes.

- **Nodes**: A, B, C, D, E
- **Edges**: Every possible connection between two nodes, resulting in 10 edges in an undirected graph.

## Types of Graphs

Graphs can be classified into several categories based on their properties:

## Legend

- **Node (Vertex)**: Represented by a letter (e.g., A, B, C). It is a fundamental part of a graph.
- **Edge**: Represented by a line connecting two nodes (e.g., A -- B). It shows a relationship between the nodes.
- **No Relationship**: When there is no line connecting two nodes, it means there is no direct relationship between them.

```plaintext
    A -- B    A and B are connected by an edge.
    C        C has no relationship with A or B.
```

1. **Undirected Graph**: A graph in which edges have no direction. The edge (A, B) is identical to the edge (B, A).
    ```plaintext
    A -- B
    ```

2. **Directed Graph (Digraph)**: A graph in which edges have a direction. The edge (A, B) is not the same as the edge (B, A).
    ```plaintext
    A -> B
    ```

3. **Weighted Graph**: A graph in which edges have weights or costs associated with them.
    ```plaintext
    A -5- B
    ```

4. **Unweighted Graph**: A graph in which edges do not have any weights.
    ```plaintext
    A -- B
    ```

These classifications help in understanding the structure and properties of graphs, which is essential for designing efficient algorithms.