type PylosResult<T> = Result<T, String>;

fn total_positions_for_layer_count(layers: u32) -> u32 {
    layers * (layers + 1) * (2 * layers + 1) / 6
}

fn size_for_layer_0_based(layer: u32) -> u32 {
    layer + 1
}

fn bit_offset_for_layer_0_based(layer: u32) -> u32 {
    total_positions_for_layer_count(layer)
}

fn offset_from_0_based_coordinates(layer: u32, x: u32, y: u32) -> u32 {
    let offset = bit_offset_for_layer_0_based(layer) + y * size_for_layer_0_based(layer) + x;
    assert!(offset < 64);
    offset
}

fn coordinates_0_based_from_offset(offset: u32) -> (u32, u32, u32) {
    let mut previous = 0u32;
    let (layer, _, layer_offset) = (0..)
        .map(|l| {
            let current = bit_offset_for_layer_0_based(l + 1);
            let result = (l, current, previous);
            previous = current;
            result
        })
        .find(|(_, bit_offset, _)| *bit_offset > offset)
        .unwrap();
    let remainder = offset - layer_offset;
    let y = remainder / size_for_layer_0_based(layer);
    let x = remainder % size_for_layer_0_based(layer);
    (layer, x, y)
}

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq, Clone)]
struct Position<'a> {
    gc: &'a GameConstants,
    offset: u32,
}

impl Position<'_> {
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
        let layer_0_based = gc.layers - layer;
        let layer_size = size_for_layer_0_based(layer_0_based);
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

        Ok(Position {
            gc,
            offset: offset_from_0_based_coordinates(layer_0_based, x - 1, y - 1),
        })
    }

    fn coordinates(&self) -> (u32, u32, u32) {
        let (layer, x, y) = coordinates_0_based_from_offset(self.offset);
        (self.gc.layers - layer, x + 1, y + 1)
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
    fn bit_offset_for_layer() {
        assert_eq!(super::bit_offset_for_layer_0_based(3), 14);
        assert_eq!(super::bit_offset_for_layer_0_based(2), 5);
        assert_eq!(super::bit_offset_for_layer_0_based(1), 1);
        assert_eq!(super::bit_offset_for_layer_0_based(0), 0);
    }

    #[test]
    fn zero_based_positions() {
        let offset = super::offset_from_0_based_coordinates(2, 1, 0);
        let (layer, x, y) = super::coordinates_0_based_from_offset(offset);
        assert_eq!(layer, 2);
        assert_eq!(x, 1);
        assert_eq!(y, 0);

        let offset = super::offset_from_0_based_coordinates(3, 3, 3);
        let (layer, x, y) = super::coordinates_0_based_from_offset(offset);
        assert_eq!(layer, 3);
        assert_eq!(x, 3);
        assert_eq!(y, 3);
    }

    #[test]
    fn can_create_a_four_layer_game() {
        GameConstants::build(4).unwrap();
    }

    #[test]
    fn cant_create_a_ten_layer_game() {
        assert!(GameConstants::build(10).is_err());
    }

    #[test]
    fn position_construction_and_getters() {
        let gc = GameConstants::build(3).unwrap();
        let position = Position::build(&gc, 1, 2, 3).unwrap();
        let (layer, x, y) = position.coordinates();
        assert_eq!(layer, 1);
        assert_eq!(x, 2);
        assert_eq!(y, 3);
    }
    #[test]
    fn valid_positions() {
        let gc = GameConstants::build(2).unwrap();
        let _ = [
            Position::build(&gc, 1, 1, 1).unwrap(),
            Position::build(&gc, 1, 1, 2).unwrap(),
            Position::build(&gc, 1, 2, 1).unwrap(),
            Position::build(&gc, 1, 2, 2).unwrap(),
            Position::build(&gc, 2, 1, 1).unwrap(),
        ];
    }
    #[test]
    fn invalid_positions() {
        let gc = GameConstants::build(2).unwrap();
        let positions = [
            Position::build(&gc, 0, 0, 0),
            Position::build(&gc, 1, 0, 0),
            Position::build(&gc, 1, 3, 1),
            Position::build(&gc, 1, 1, 3),
            Position::build(&gc, 3, 1, 1),
        ];

        assert_eq!(None, positions.into_iter().find(|p| p.is_ok()));
    }
}
