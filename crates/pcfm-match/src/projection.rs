use crate::spatial::{PlayerSpatialState, SpatialState};
use crate::tactics::{PhaseState, TeamSide};

/// A single projected player on the 2D field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerProjection {
    /// Team side.
    pub team: TeamSide,
    /// Row in the projected grid.
    pub row: usize,
    /// Column in the projected grid.
    pub col: usize,
    /// Marker used for the player.
    pub marker: char,
    /// Role name for display.
    pub role: &'static str,
}

/// A projected view of the match on a 2D pitch.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldProjection {
    /// Width of the rendered field in columns.
    pub width: usize,
    /// Height of the rendered field in rows.
    pub height: usize,
    /// Ball position in the projected grid.
    pub ball: Option<(usize, usize)>,
    /// Projected players.
    pub players: Vec<PlayerProjection>,
    /// The match phase, if available.
    pub phase: Option<PhaseState>,
    /// Team currently in possession, if known.
    pub possession: Option<TeamSide>,
}

/// A renderer that projects the authoritative spatial state into a 2D ASCII field.
#[derive(Debug, Clone, Default)]
pub struct FieldRenderer {
    width: usize,
    height: usize,
}

impl FieldRenderer {
    /// Creates a projection renderer with a conventional pitch size.
    #[must_use]
pub fn new() -> Self {
        Self {
            width: 40,
            height: 22,
        }
    }

    /// Projects a spatial state into a 2D view.
    #[must_use]
    pub fn project(
        &self,
        spatial: &SpatialState,
        phase: Option<PhaseState>,
        possession: Option<TeamSide>,
    ) -> FieldProjection {
        let width = self.width;
        let height = self.height;

        let ball = spatial
            .ball
            .position
            .x
            .is_finite()
            .then_some(())
            .and_then(|_| {
                let row = to_row(spatial.ball.position.y, height);
                let col = to_col(spatial.ball.position.x, width);
                Some((row, col))
            });

        let players = spatial
            .players
            .iter()
            .map(|player| project_player(player, width, height))
            .collect();

        FieldProjection {
            width,
            height,
            ball,
            players,
            phase,
            possession,
        }
    }

    /// Renders the field projection as an ASCII grid.
    #[must_use]
    pub fn render_ascii(&self, projection: &FieldProjection) -> String {
        let mut grid = vec![vec![' '; self.width]; self.height];

        for row in 0..self.height {
            for col in 0..self.width {
                if row == self.height / 2 || col == self.width / 2 {
                    grid[row][col] = '·';
                }
            }
        }

        if let Some((row, col)) = projection.ball {
            if row < self.height && col < self.width {
                grid[row][col] = 'O';
            }
        }

        for player in &projection.players {
            if player.row < self.height && player.col < self.width {
                grid[player.row][player.col] = player.marker;
            }
        }

        let mut lines = Vec::with_capacity(self.height);
        for row in grid {
            lines.push(row.into_iter().collect::<String>());
        }

        let mut out = String::new();
        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                out.push('\n');
            }
            out.push_str(line);
        }
        out
    }
}

impl Default for FieldRenderer {
    fn default() -> Self {
        Self::new()
    }
}

fn project_player(player: &PlayerSpatialState, width: usize, height: usize) -> PlayerProjection {
    let marker = if player.role.starts_with("goal") {
        'G'
    } else if player.role.contains("def") || player.role.contains("back") {
        'D'
    } else if player.role.contains("mid") {
        'M'
    } else if player.role.contains("wing") || player.role.contains("att") {
        'W'
    } else {
        'P'
    };

    let team = if player.player_id.slot() % 2 == 0 {
        TeamSide::Home
    } else {
        TeamSide::Away
    };

    PlayerProjection {
        team,
        row: to_row(player.position.y, height),
        col: to_col(player.position.x, width),
        marker,
        role: player.role,
    }
}

fn to_row(value: f32, height: usize) -> usize {
    let clamped = value.clamp(-1.0, 1.0);
    let row = ((clamped + 1.0) * (height as f32 - 1.0) / 2.0).round() as usize;
    row.min(height.saturating_sub(1))
}

fn to_col(value: f32, width: usize) -> usize {
    let clamped = value.clamp(-1.0, 1.0);
    let col = ((clamped + 1.0) * (width as f32 - 1.0) / 2.0).round() as usize;
    col.min(width.saturating_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial::{
        BallState, PlayerSpatialState, PressureMap, PressurePoint, SpatialState, TeamShapeState,
        Vec2,
    };

    #[test]
    fn field_projection_renders_without_panicking() {
        let spatial = SpatialState {
            ball: BallState {
                position: Vec2 { x: 0.0, y: 0.0 },
                velocity: Vec2 { x: 0.0, y: 0.0 },
                owner: Some(1),
                last_touch: Some(1),
            },
            players: vec![
                PlayerSpatialState {
                    player_id: 1,
                    position: Vec2 { x: 0.2, y: 0.2 },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    heading: 0.0,
                    role: "midfielder",
                    zone: "middle",
                    spacing_bias: 0.5,
                },
                PlayerSpatialState {
                    player_id: 2,
                    position: Vec2 { x: -0.2, y: -0.2 },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    heading: 0.0,
                    role: "defender",
                    zone: "left",
                    spacing_bias: 0.5,
                },
            ],
            home_shape: TeamShapeState {
                width: 1.0,
                depth: 1.0,
                compactness: 1.0,
                line_height: 0.5,
                left_anchor: -0.5,
                right_anchor: 0.5,
                press_line: 0.4,
            },
            away_shape: TeamShapeState {
                width: 1.0,
                depth: 1.0,
                compactness: 1.0,
                line_height: 0.5,
                left_anchor: -0.5,
                right_anchor: 0.5,
                press_line: 0.4,
            },
            pressure: PressureMap {
                home: vec![PressurePoint {
                    position: Vec2 { x: 0.0, y: 0.0 },
                    intensity: 0.5,
                }],
                away: vec![PressurePoint {
                    position: Vec2 { x: 0.0, y: 0.0 },
                    intensity: 0.5,
                }],
            },
        };

        let renderer = FieldRenderer::new();
        let projection = renderer.project(&spatial, None, Some(TeamSide::Home));
        let txt = renderer.render_ascii(&projection);
        assert!(txt.contains('O'));
        assert!(txt.contains('M') || txt.contains('D'));
    }
}
