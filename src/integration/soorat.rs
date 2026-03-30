//! Soorat integration — visualization data structures for creature behavior.
//!
//! Provides structured types that soorat can render: agent positions,
//! territory boundaries, migration paths, social graphs, and swarm fields.

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// ── Agent positions ────────────────────────────────────────────────────────

/// Creature agent data for instanced particle/sprite rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentField {
    /// Agent states.
    pub agents: Vec<AgentViz>,
}

/// A single creature agent for rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentViz {
    /// World-space position `[x, y, z]`.
    pub position: [f32; 3],
    /// Heading direction `[dx, dy, dz]` (unit vector).
    pub heading: [f32; 3],
    /// Current behavioral state name (e.g. "Foraging", "Fleeing", "Resting").
    pub state: String,
    /// Speed (m/s).
    pub speed: f32,
    /// Alertness level (0.0–1.0) for visual indicator.
    pub alertness: f32,
}

// ── Territory boundaries ───────────────────────────────────────────────────

/// Territory region data for colored overlay rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerritoryMap {
    /// Territory regions.
    pub territories: Vec<TerritoryRegion>,
}

/// A single territory region.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerritoryRegion {
    /// Center position `[x, z]` in world space.
    pub center: [f32; 2],
    /// Approximate radius (m).
    pub radius: f32,
    /// Owner agent index.
    pub owner_id: u32,
    /// Territory strength (0.0–1.0) for opacity.
    pub strength: f32,
}

// ── Migration paths ────────────────────────────────────────────────────────

/// Migration waypoint sequence for animated line rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationPath {
    /// Waypoints `[x, y, z]` along the migration route.
    pub waypoints: Vec<[f32; 3]>,
    /// Timestamp (normalized 0.0–1.0 through the year) at each waypoint.
    pub timestamps: Vec<f32>,
    /// Species or group label.
    pub label: String,
}

// ── Social graph ───────────────────────────────────────────────────────────

/// Social relationship graph for node-link visualization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialGraph {
    /// Agents as nodes.
    pub nodes: Vec<SocialNode>,
    /// Relationships as edges.
    pub edges: Vec<SocialEdge>,
}

/// A node in the social graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialNode {
    /// Agent index.
    pub id: u32,
    /// Position `[x, z]` for layout.
    pub position: [f32; 2],
    /// Social rank (0.0 = lowest, 1.0 = alpha).
    pub rank: f32,
    /// Role label (e.g. "Alpha", "Subordinate", "Sentinel").
    pub role: String,
}

/// An edge in the social graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialEdge {
    /// Source agent index.
    pub from: u32,
    /// Target agent index.
    pub to: u32,
    /// Relationship type (e.g. "Dominance", "Affiliation", "Kin").
    pub relation: String,
    /// Strength (0.0–1.0) for edge width.
    pub strength: f32,
}

// ── Swarm density field ────────────────────────────────────────────────────

/// Swarm density/velocity field for heatmap/flow rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwarmField {
    /// Density at each grid point (agents per cell).
    /// Flattened row-major: `density[z * nx + x]`.
    pub density: Vec<f32>,
    /// Average velocity at each grid point `[vx, vz]`.
    pub velocity: Vec<[f32; 2]>,
    /// Grid dimensions (nx, nz).
    pub dimensions: [usize; 2],
    /// Cell size in metres.
    pub cell_size: f32,
    /// Maximum density for normalization.
    pub max_density: f32,
}

impl SwarmField {
    /// Create from a list of agent positions and velocities on a grid.
    #[must_use]
    pub fn from_agents(
        positions: &[[f32; 2]],
        velocities: &[[f32; 2]],
        grid_min: [f32; 2],
        grid_max: [f32; 2],
        cell_size: f32,
    ) -> Self {
        if cell_size <= 0.0 {
            return Self {
                density: Vec::new(),
                velocity: Vec::new(),
                dimensions: [0, 0],
                cell_size,
                max_density: 0.0,
            };
        }
        let nx = ((grid_max[0] - grid_min[0]) / cell_size).ceil() as usize;
        let nz = ((grid_max[1] - grid_min[1]) / cell_size).ceil() as usize;
        let count = nx.max(1) * nz.max(1);

        let mut density = vec![0.0_f32; count];
        let mut vel_sum = vec![[0.0_f32; 2]; count];

        let n = positions.len().min(velocities.len());
        for i in 0..n {
            let ix = ((positions[i][0] - grid_min[0]) / cell_size) as usize;
            let iz = ((positions[i][1] - grid_min[1]) / cell_size) as usize;
            if ix < nx && iz < nz {
                let idx = iz * nx + ix;
                density[idx] += 1.0;
                vel_sum[idx][0] += velocities[i][0];
                vel_sum[idx][1] += velocities[i][1];
            }
        }

        let mut max_density = 0.0_f32;
        let velocity: Vec<[f32; 2]> = density
            .iter()
            .zip(vel_sum.iter())
            .map(|(&d, &vs)| {
                if d > max_density {
                    max_density = d;
                }
                if d > 0.0 {
                    [vs[0] / d, vs[1] / d]
                } else {
                    [0.0, 0.0]
                }
            })
            .collect();

        Self {
            density,
            velocity,
            dimensions: [nx, nz],
            cell_size,
            max_density,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_field_manual() {
        let field = AgentField {
            agents: vec![AgentViz {
                position: [1.0, 0.0, 2.0],
                heading: [1.0, 0.0, 0.0],
                state: String::from("Foraging"),
                speed: 1.5,
                alertness: 0.3,
            }],
        };
        assert_eq!(field.agents.len(), 1);
    }

    #[test]
    fn territory_map_manual() {
        let map = TerritoryMap {
            territories: vec![TerritoryRegion {
                center: [50.0, 50.0],
                radius: 20.0,
                owner_id: 0,
                strength: 0.8,
            }],
        };
        assert_eq!(map.territories.len(), 1);
    }

    #[test]
    fn migration_path_manual() {
        let path = MigrationPath {
            waypoints: vec![[0.0, 0.0, 0.0], [100.0, 0.0, 50.0]],
            timestamps: vec![0.0, 0.5],
            label: String::from("caribou"),
        };
        assert_eq!(path.waypoints.len(), 2);
    }

    #[test]
    fn social_graph_manual() {
        let graph = SocialGraph {
            nodes: vec![
                SocialNode {
                    id: 0,
                    position: [0.0, 0.0],
                    rank: 1.0,
                    role: String::from("Alpha"),
                },
                SocialNode {
                    id: 1,
                    position: [1.0, 0.0],
                    rank: 0.5,
                    role: String::from("Beta"),
                },
            ],
            edges: vec![SocialEdge {
                from: 0,
                to: 1,
                relation: String::from("Dominance"),
                strength: 0.9,
            }],
        };
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn swarm_field_from_agents() {
        let positions = [[1.0, 1.0], [1.5, 1.5], [5.0, 5.0]];
        let velocities = [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0]];
        let field = SwarmField::from_agents(&positions, &velocities, [0.0, 0.0], [10.0, 10.0], 2.0);
        assert_eq!(field.dimensions, [5, 5]);
        assert!(field.max_density >= 1.0);
    }

    #[test]
    fn swarm_field_empty() {
        let field = SwarmField::from_agents(&[], &[], [0.0, 0.0], [10.0, 10.0], 2.0);
        assert_eq!(field.max_density, 0.0);
    }

    #[test]
    fn swarm_field_zero_cell_size() {
        let field = SwarmField::from_agents(&[], &[], [0.0, 0.0], [10.0, 10.0], 0.0);
        assert!(field.density.is_empty());
    }

    #[test]
    fn agent_field_serializes() {
        let field = AgentField { agents: Vec::new() };
        let json = serde_json::to_string(&field);
        assert!(json.is_ok());
    }
}
