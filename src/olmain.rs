fn main() {
    let mut graph = Graph {
        vertices: Vec::new(),
        edges: Vec::new(),
    };

    let v1 = Vertex {
        name: String::from("v1"),
    };
    let v2 = Vertex {
        name: String::from("v2"),
    };
    let v3 = Vertex {
        name: String::from("v3"),
    };

    // add vertices to graph
    addV(&mut graph, v1.clone());
    addV(&mut graph, v2.clone());
    addV(&mut graph, v3.clone());

    // add edges to graph
    addE(&mut graph, String::from("edge_1"), v1.clone(), v2.clone());
    addE(&mut graph, String::from("edge_2"), v2.clone(), v3.clone());

    // print graph
    println!("{:?}", graph);
}

/**
 * Vertex structure
 * It is a fundamental unit to which edges are attached in a graph database. It represents an entity or object in the graph and can store data associated with that entity.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
struct Vertex {
    name: String,
}

/**
 * Edge structure
 * It represents a connection between two vertices in a graph database. It can be used to store additional data about the relationship between the two vertices it connects.
 */
#[derive(Debug, Clone)]
struct Edge {
    label: String,
    from: Vertex,
    to: Vertex,
}

/**
 * Graph structure
 * Graph is a data structure that consists of vertices (also known as nodes) and edges. It is used to represent relationships between objects or entities, and is often used for data that is too complex or does not fit well into a traditional relational database structure.
 */
#[derive(Debug, Clone)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

// addV() add a vertex to the graph
fn addV(graph: &mut Graph, vertex: Vertex) {
    graph.vertices.push(vertex);
}

// addE(‘label’).from(vertex).to(vertex) => create an edge
fn addE(graph: &mut Graph, label: String, from: Vertex, to: Vertex) {
    graph.edges.push(Edge { label, from, to });
}

// V() return all vertices
fn V(graph: &Graph) -> &Vec<Vertex> {
    &graph.vertices
}

// E() return all edges
fn E(graph: &Graph) -> &Vec<Edge> {
    &graph.edges
}

// V().has(property-name, property-value) // return a collection or an iterator (up to you) of vertices that has the property property-name and the property-value associated
fn V_has(graph: &Graph, property_name: &str, property_value: &str) -> Vec<Vertex> {
    let mut result = Vec::new();
    for vertex in &graph.vertices {
        if vertex.name == property_value {
            result.push(vertex.clone());
        }
    }
    result
}

// outE() and inE() return outgoing or ingoing vertices
fn outE(graph: &Graph, vertex: Vertex) -> Vec<Edge> {
    let mut result = Vec::new();
    for edge in &graph.edges {
        if edge.from == vertex {
            result.push(edge.clone());
        }
    }
    result
}

fn inE(graph: &Graph, vertex: Vertex) -> Vec<Edge> {
    let mut result = Vec::new();
    for edge in &graph.edges {
        if edge.to == vertex {
            result.push(edge.clone());
        }
    }
    result
}

// drop() remove a vertex (including ingoing and outgoing relationships)

// Optional:
// Try to implement repeat(times)
// Try to implement until(traversal)
