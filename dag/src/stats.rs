use std::fmt;
pub trait DAGStats {
    fn calculate(&self) -> DAGStatsData;
    fn get_average_depth(&self) -> f64;
    fn get_average_nodes_per_depth(&self) -> f64;
    fn get_average_in_references(&self) -> f64;
    fn get_max_depth(&self) -> usize;
    fn get_nodes_count(&self) -> usize;
    fn get_edges_count(&self) -> usize;
    fn get_nodes_with_no_incoming_edges(&self) -> usize;
    fn get_isolated_nodes_count(&self) -> usize;
}

impl fmt::Display for DAGStatsData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DAG Statistics:")?;
        writeln!(f, "Average Depth: {}", self.average_depth.unwrap_or(0.0))?;
        writeln!(
            f,
            "Average Nodes per Depth: {}",
            self.average_nodes_per_depth.unwrap_or(0.0)
        )?;
        writeln!(
            f,
            "Average In-References: {}",
            self.average_in_references.unwrap_or(0.0)
        )?;
        writeln!(f, "Max Depth: {}", self.max_depth.unwrap_or(0))?;
        writeln!(f, "Nodes Count: {}", self.nodes_count.unwrap_or(0))?;
        writeln!(f, "Edges Count: {}", self.edges_count.unwrap_or(0))?;
        writeln!(
            f,
            "Nodes with No Incoming Edges: {}\n",
            self.nodes_with_no_incoming_edges.unwrap_or(0)
        )?;
        writeln!(
            f,
            "Isolated Nodes Count: {}",
            self.isolated_nodes_count.unwrap_or(0)
        )
    }
}
#[derive(Default)]
pub struct DAGStatsData {
    pub average_depth: Option<f64>,
    pub average_nodes_per_depth: Option<f64>,
    pub average_in_references: Option<f64>,
    pub max_depth: Option<usize>,
    pub nodes_count: Option<usize>,
    pub edges_count: Option<usize>,
    pub nodes_with_no_incoming_edges: Option<usize>,
    pub isolated_nodes_count: Option<usize>,
}
impl Default for DAGStatsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DAGStatsData {
    pub fn new() -> Self {
        DAGStatsData::default()
    }
}


pub struct DAGStatsBuilder {
    stats: DAGStatsData,
}

impl DAGStatsBuilder {
    pub fn new() -> Self {
        DAGStatsBuilder {
            stats: DAGStatsData::default(),
        }
    }

    pub fn set_average_depth(&mut self, depth: f64) -> &mut Self {
        self.stats.average_depth = Some(depth);
        self
    }

    pub fn set_average_nodes_per_depth(&mut self, nodes_per_depth: f64) -> &mut Self {
        self.stats.average_nodes_per_depth = Some(nodes_per_depth);
        self
    }

    pub fn set_average_in_references(&mut self, in_references: f64) -> &mut Self {
        self.stats.average_in_references = Some(in_references);
        self
    }

    pub fn set_max_depth(&mut self, max_depth: usize) -> &mut Self {
        self.stats.max_depth = Some(max_depth);
        self
    }

    pub fn set_nodes_count(&mut self, nodes_count: usize) -> &mut Self {
        self.stats.nodes_count = Some(nodes_count);
        self
    }

    pub fn set_edges_count(&mut self, edges_count: usize) -> &mut Self {
        self.stats.edges_count = Some(edges_count);
        self
    }

    pub fn set_nodes_with_no_incoming_edges(&mut self, no_incoming_edges: usize) -> &mut Self {
        self.stats.nodes_with_no_incoming_edges = Some(no_incoming_edges);
        self
    }

    pub fn set_isolated_nodes_count(&mut self, isolated_nodes_count: usize) -> &mut Self {
        self.stats.isolated_nodes_count = Some(isolated_nodes_count);
        self
    }

    pub fn build(&self) -> DAGStatsData {
        DAGStatsData {
            average_depth: self.stats.average_depth,
            average_nodes_per_depth: self.stats.average_nodes_per_depth,
            average_in_references: self.stats.average_in_references,
            max_depth: self.stats.max_depth,
            nodes_count: self.stats.nodes_count,
            edges_count: self.stats.edges_count,
            nodes_with_no_incoming_edges: self.stats.nodes_with_no_incoming_edges,
            isolated_nodes_count: self.stats.isolated_nodes_count,
        }
    }
}
