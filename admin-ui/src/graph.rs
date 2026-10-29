//! Graph data from `/api/graph` and a small force-directed layout.

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct NodeData {
    pub id: i64,
    pub trace_id: String,
    pub partition: String,
    pub instance: String,
    pub role: String,
    pub content: Option<String>,
    pub timestamp: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SynapseData {
    pub from: i64,
    pub to: i64,
    pub score: Option<f64>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct GraphData {
    pub nodes: Vec<NodeData>,
    pub synapses: Vec<SynapseData>,
}

pub struct Graph {
    pub nodes: Vec<NodeData>,
    pub pos: Vec<[f32; 2]>,
    vel: Vec<[f32; 2]>,
    /// Messages that share a trace id, chained in time order.
    pub trace_links: Vec<(usize, usize)>,
    pub synapse_links: Vec<(usize, usize, f32)>,
    ticks: u32,
}

const LAYOUT_TICKS: u32 = 600;

impl Graph {
    pub fn new(data: &GraphData, partition: Option<&str>) -> Self {
        let mut nodes: Vec<NodeData> = data
            .nodes
            .iter()
            .filter(|n| partition.is_none_or(|p| n.partition == p))
            .cloned()
            .collect();
        nodes.sort_by_key(|n| n.timestamp);
        let index: HashMap<i64, usize> = nodes.iter().enumerate().map(|(i, n)| (n.id, i)).collect();

        let mut by_trace: HashMap<&str, Vec<usize>> = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            by_trace.entry(&n.trace_id).or_default().push(i);
        }
        let mut trace_links = Vec::new();
        for ids in by_trace.values() {
            for w in ids.windows(2) {
                trace_links.push((w[0], w[1]));
            }
        }
        let synapse_links = data
            .synapses
            .iter()
            .filter_map(|s| {
                Some((*index.get(&s.from)?, *index.get(&s.to)?, s.score.unwrap_or(0.5) as f32))
            })
            .collect();

        // Start on a spiral so the layout is deterministic and not degenerate.
        let pos = (0..nodes.len())
            .map(|i| {
                let a = i as f32 * 2.4;
                let r = 12.0 * (i as f32).sqrt();
                [r * a.cos(), r * a.sin()]
            })
            .collect();
        Self {
            vel: vec![[0.0; 2]; nodes.len()],
            nodes,
            pos,
            trace_links,
            synapse_links,
            ticks: 0,
        }
    }

    /// Advances the layout one tick. Returns true while it is still settling.
    pub fn step(&mut self) -> bool {
        if self.ticks >= LAYOUT_TICKS || self.nodes.is_empty() {
            return false;
        }
        self.ticks += 1;
        let n = self.pos.len();
        let cooling = 1.0 - self.ticks as f32 / LAYOUT_TICKS as f32;
        let mut force = vec![[0.0f32; 2]; n];

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = self.pos[i][0] - self.pos[j][0];
                let dy = self.pos[i][1] - self.pos[j][1];
                let d2 = (dx * dx + dy * dy).max(1.0);
                let f = 900.0 / d2;
                let d = d2.sqrt();
                force[i][0] += dx / d * f;
                force[i][1] += dy / d * f;
                force[j][0] -= dx / d * f;
                force[j][1] -= dy / d * f;
            }
            // Gentle pull to the middle keeps disconnected pieces on screen.
            force[i][0] -= self.pos[i][0] * 0.01;
            force[i][1] -= self.pos[i][1] * 0.01;
        }
        let links = self
            .trace_links
            .iter()
            .map(|&(a, b)| (a, b, 0.02))
            .chain(self.synapse_links.iter().map(|&(a, b, _)| (a, b, 0.05)));
        for (a, b, k) in links {
            let dx = self.pos[b][0] - self.pos[a][0];
            let dy = self.pos[b][1] - self.pos[a][1];
            let d = (dx * dx + dy * dy).sqrt().max(0.01);
            let f = (d - 40.0) * k;
            force[a][0] += dx / d * f;
            force[a][1] += dy / d * f;
            force[b][0] -= dx / d * f;
            force[b][1] -= dy / d * f;
        }
        for i in 0..n {
            for k in 0..2 {
                self.vel[i][k] = (self.vel[i][k] + force[i][k]) * 0.8;
                self.pos[i][k] += self.vel[i][k].clamp(-20.0, 20.0) * cooling.max(0.05);
            }
        }
        true
    }
}
