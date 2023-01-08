fn main() {
    let mut graph = Graph {
        vertices: Vec::new(),
        edges: Vec::new(),
    };

    graph
        .addV(Vertex {
            name: String::from("v1"),
        })
        .addV(Vertex {
            name: String::from("v2"),
        })
        .addV(Vertex {
            name: String::from("v3"),
        });

    graph.addE(Edge {
        label: String::from("edge_1"),
        from: graph.V()[0],
        to: graph.V()[1],
    });

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

impl Graph {
    // addV() add a vertex to the graph
    fn addV(&mut self, vertex: Vertex) -> &Graph {
        self.vertices.push(vertex);
        self
    }

    // addE(‘label’).from(vertex).to(vertex) => create an edge
    fn addE(&mut self, edge: Edge) -> &Graph {
        self.edges.push(edge);
        self
    }

    // V() return all vertices
    fn V(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    // E() return all edges
    fn E(&self) -> &Vec<Edge> {
        &self.edges
    }

    // V().has(property-name, property-value) // return a collection or an iterator (up to you) of vertices that has the property property-name and the property-value associated
    fn V_has(&self, property_name: &str, property_value: &str) -> Vec<Vertex> {
        let mut result = Vec::new();
        for vertex in self.V() {
            if vertex.name == property_value {
                result.push(vertex.clone());
            }
        }
        result
    }

    // outE() and inE() return outgoing or ingoing vertices
    fn outE(&self, vertex: Vertex) -> Vec<Edge> {
        let mut result = Vec::new();
        for edge in self.E() {
            if edge.from == vertex {
                result.push(edge.clone());
            }
        }
        result
    }

    fn inE(&self, vertex: Vertex) -> Vec<Edge> {
        let mut result = Vec::new();
        for edge in self.E() {
            if edge.to == vertex {
                result.push(edge.clone());
            }
        }
        result
    }
}
