use crate::PylosResult;

pub fn total_positions_for_layer_count(layers: u32) -> u32 {
    layers * (layers + 1) * (2 * layers + 1) / 6
}

pub fn size_for_layer_0_based(layer: u32) -> u32 {
    layer + 1
}

fn bit_offset_for_layer_0_based(layer: u32) -> u32 {
    total_positions_for_layer_count(layer)
}

fn all_positions_for_layer_0_based(layer: u32) -> u64 {
    let start = total_positions_for_layer_count(layer);
    let finish = total_positions_for_layer_count(layer + 1);

    (1 << (finish - start) - 1) << start
}

pub fn offset_from_0_based_coordinates(layer: u32, x: u32, y: u32) -> u32 {
    let offset = bit_offset_for_layer_0_based(layer) + y * size_for_layer_0_based(layer) + x;
    assert!(offset < 64);
    offset
}

pub fn coordinates_0_based_from_offset(offset: u32) -> (u32, u32, u32) {
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
pub struct GameConstants {
    pub layers: u32,
}

impl GameConstants {
    pub fn build(layers: u32) -> PylosResult<GameConstants> {
        let total_balls = total_positions_for_layer_count(layers);
        if total_balls > 64 {
            return Err(format!("A game with {layers} layers needs {total_balls} balls, which is more than supported"));
        }

        Ok(GameConstants { layers })
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinate_conversions::GameConstants;

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
}
