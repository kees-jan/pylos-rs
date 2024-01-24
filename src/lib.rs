type PylosResult<T> = Result<T, String>;

fn total_positions_for_layer_count(layers: u32) -> u32 {
    layers * (layers + 1) * (2 * layers + 1) / 6
}

struct GameConstants {
    layers: u32,
}

impl GameConstants {
    fn build(layers: u32) -> PylosResult<GameConstants> {
        let total_balls = total_positions_for_layer_count(layers);
        if total_balls > 64 {
            return Err(format!("A game with {layers} layers needs {total_balls} balls, which is more than supported"));
        }

        Ok(GameConstants { layers })
    }

    fn size_for_layer(&self, layer: u32) -> u32 {
        self.layers - layer
    }

    fn bit_offset_for_layer(&self, layer: u32) -> u32 {
        total_positions_for_layer_count(self.layers - 1 - layer)
    }
}

struct Position {
    gc: &GameConstants,
    offset: u32,
}

impl Position {
    fn build(gc: &GameConstants, layer: u32, x: u32, y: u32) -> PylosResult<Position> {
        if layer > gc.layers {
            return Err(format!(
                "In a game with {} layers, there is no position on layer {}",
                gc.layers, layer
            ));
        }
        if layer == 0 {
            return Err(String::from("Layer 0 does not exist"));
        }

        let layer_size = gc.size_for_layer(layer - 1);
        if x == 0 || x > layer_size {
            return Err(format!(
                "x-coordinate {} is outside range 1..{}",
                x, layer_size
            ));
        }
        if y == 0 || y > layer_size {
            return Err(format!(
                "y-coordinate {} is outside range 1..{}",
                y, layer_size
            ));
        }

        Ok(Position::build_0_based(gc, layer - 1, x - 1, y - 1))
    }

    fn build_0_based(gc: &GameConstants, layer: u32, x: u32, y: u32) -> Position {
        let offset = gc.bit_offset_for_layer(layer) + y * gc.layers + x;

        assert!(offset < 64);

        Position { offset }
    }

    fn coordinates(&self, gc: &GameConstants) -> (u32, u32, u32) {
        let (layer, x, y) = self.coordinates_0_based();
        (layer + 1, x + 1, y + 1)
    }

    fn coordinates_0_based(&self) -> (u32, u32, u32) {
        todo!()
    }
}

struct PositionSet {
    positions: u64,
}

impl PositionSet {}

struct Board {
    white_pieces: PositionSet,
    black_pieces: PositionSet,
}

impl Board {}

#[cfg(test)]
mod tests {
    use crate::{GameConstants, Position};

    #[test]
    fn can_create_a_four_layer_game() {
        GameConstants::build(4).unwrap();
    }

    #[test]
    fn cant_create_a_ten_layer_game() {
        assert!(GameConstants::build(10).is_err());
    }

    #[test]
    fn bit_offset_for_layer() {
        let gc = GameConstants::build(4).unwrap();

        assert_eq!(gc.bit_offset_for_layer(3), 0);
        assert_eq!(gc.bit_offset_for_layer(2), 1);
        assert_eq!(gc.bit_offset_for_layer(1), 5);
        assert_eq!(gc.bit_offset_for_layer(0), 14);
    }

    #[test]
    fn position_construction_and_getters() {
        let gc = GameConstants::build(3).unwrap();
        let position = Position::build(&gc, 1, 2, 3).unwrap();
        let (layer, x, y) = position.coordinates();
        assert_eq!(layer, 1);
        assert_eq!(x, 2);
        assert_eq!(y, 3);
        let (layer, x, y) = position.coordinates_0_based();
        assert_eq!(layer, 0);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    }
    #[test]
    fn valid_positions() {
        let gc = GameConstants::build(2).unwrap();
        let positions = [
            Position::build(&gc, 1, 1, 1),
            Position::build(&gc, 1, 1, 2),
            Position::build(&gc, 1, 2, 1),
            Position::build(&gc, 1, 2, 2),
            Position::build(&gc, 2, 1, 1),
        ];
    }
    #[test]
    fn invalid_positions() {
        let gc = GameConstants::build(2).unwrap();
        let positions = [Position::build(&gc, 0, 0, 9)];
    }
}
