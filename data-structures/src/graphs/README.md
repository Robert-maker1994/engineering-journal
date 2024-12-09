# Graphs 

A graph is a non-linear data structure made up of vertices and edges. The nodes known as vertices are connected by edges.

The definition of a graph is `G = (v,e)`
Graphs are used in a loads of different problems and applications all over the internet, take social media for example, how people are connected are through graphs. Social networks can be represented as a graph, with a person being a vertices and a edge being the connection between two people. 

## Terminology 

1. **Vertex**: A fundamental part of the graph representing an entity or a point.
2. **Edge**: An connection between two vertices. 
3. **Degree**: The number of edges connected to a vertices. 
    - **In-degree**: The number of incoming edges to a vertex. 
    - **Out-degree**: The number of outgoing edges. 
4. **Path**: A sequence of vertices where each adjacent pair is connected by an edge.
    **Path Length**: What is the path length from vertices A -> B. 
5. **Cycle**: A path that starts and ends in the same vertices 
6. **Connectivity**: All graphs have some kind of connectivity. 
    - **Connected Graphs**: There is a path between any pair of vertices. 
    - **Disconnected Graphs**: Least one pair of vertices without a path between them.


## Difference between a weighted and unweighted graph. 

- **Weighted graph**: The edges are labeled with their corresponding weights, allowing algorithms to take these weights into account when performing operations like finding the shortest path, minimum spanning tree, or maximum flow.

- **Unweighted Graph**: An unweighted graph, also known as an ordinary graph, is a graph where each edge is simply present or absent, without any numerical weight or cost associated with it. Unweighted graphs are used to model relationships that are either present or absent, without considering the magnitude or strength of the connection.


## How do you represent a graph as a data structure?

1. **The adjacency Matrix**

### Example of an Adjacency Matrix

Consider the following graph:

```
A -- B
|  / |
| /  |
C -- D
```

The adjacency matrix for this graph would be:

|   | A | B | C | D |
|---|---|---|---|---|
| A | 0 | 1 | 1 | 0 |
| B | 1 | 0 | 1 | 1 |
| C | 1 | 1 | 0 | 1 |
| D | 0 | 1 | 1 | 0 |

In this matrix, the rows and columns represent the vertices, and the values (0 or 1) indicate whether there is an edge between the vertices. For example, the value at row A and column B is 1, indicating there is an edge between vertex A and vertex B.

2. **Adjacency List**

### Example of an Adjacency List

An adjacency list represents a graph as a collection of lists. The keys are the vertices, and the values are lists of adjacent vertices.

Consider the same graph:

```
A -- B
|  / |
| /  |
C -- D
```

The adjacency list for this graph would be:

```
A: B, C
B: A, C, D
C: A, B, D
D: B, C
```

In this list, each vertex points to a list of vertices that it is connected to. For example, vertex A is connected to vertices B and C.

3. **Edge Set**: An edge list is a list of all edges in the graph. Each edge is represented as a pair (or tuple) of vertices.

### Example of an Edge Set

An edge set represents a graph as a collection of edges. Each edge is represented as a pair (or tuple) of vertices.

Consider the same graph:

```
A -- B
|  / |
| /  |
C -- D
```

The edge set for this graph would be:

```
{ (A, B), (A, C), (B, C), (B, D), (C, D) }
```

In this set, each pair represents an edge between two vertices. For example, the pair (A, B) indicates there is an edge between vertex A and vertex B.

