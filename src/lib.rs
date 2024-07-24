use crate::coordinate_conversions::{
    coordinates_0_based_from_offset, offset_from_0_based_coordinates, size_for_layer_0_based,
    GameConstants,
};

pub type PylosResult<T> = Result<T, String>;

mod coordinate_conversions;

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

#[derive(Debug, PartialEq, Clone)]
struct PositionSet {
    positions: u64,
}

impl PositionSet {
    fn new() -> PositionSet {
        PositionSet { positions: 0 }
    }
    fn contains(&self, p: &Position) -> bool {
        !matches!(self.positions & (1 << p.offset), 0)
    }
    fn insert(&mut self, p: &Position) {
        self.positions |= 1 << p.offset;
    }
    fn remove(&mut self, p: &Position) {
        self.positions &= !(1 << p.offset);
    }

    fn union(&self, other: &PositionSet) -> PositionSet {
        PositionSet {
            positions: self.positions | other.positions,
        }
    }

    fn intersection(&self, other: &PositionSet) -> PositionSet {
        PositionSet {
            positions: self.positions & other.positions,
        }
    }

    fn difference(&self, other: &PositionSet) -> PositionSet {
        PositionSet {
            positions: self.positions & !other.positions,
        }
    }
}

struct Board {
    white_pieces: PositionSet,
    black_pieces: PositionSet,
}

impl Board {}

#[cfg(test)]
mod tests {
    use crate::coordinate_conversions::GameConstants;
    use crate::{Position, PositionSet};

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

    #[test]
    fn can_add_position_to_set() {
        let gc = GameConstants::build(2).unwrap();
        let p1 = Position::build(&gc, 2, 1, 1).unwrap();
        let p2 = Position::build(&gc, 1, 2, 1).unwrap();
        let mut position_set = PositionSet::new();
        position_set.insert(&p1);
        assert!(position_set.contains(&p1));
        assert!(!position_set.contains(&p2));
        position_set.insert(&p2);
        assert!(position_set.contains(&p1));
        assert!(position_set.contains(&p2));
    }

    #[test]
    fn can_remove_position_from_set() {
        let gc = GameConstants::build(2).unwrap();
        let p1 = Position::build(&gc, 2, 1, 1).unwrap();
        let p2 = Position::build(&gc, 1, 2, 1).unwrap();
        let mut position_set = PositionSet::new();
        position_set.insert(&p1);
        position_set.insert(&p2);
        assert!(position_set.contains(&p1));
        assert!(position_set.contains(&p2));
        position_set.remove(&p2);
        assert!(position_set.contains(&p1));
        assert!(!position_set.contains(&p2));
    }

    #[test]
    fn set_union() {
        let gc = GameConstants::build(2).unwrap();
        let p1 = Position::build(&gc, 2, 1, 1).unwrap();
        let p2 = Position::build(&gc, 1, 2, 1).unwrap();
        let p3 = Position::build(&gc, 1, 1, 1).unwrap();
        let p4 = Position::build(&gc, 1, 1, 2).unwrap();
        let mut set1 = PositionSet::new();
        let mut set2 = PositionSet::new();
        set1.insert(&p1);
        set1.insert(&p2);
        set2.insert(&p2);
        set2.insert(&p3);
        assert_eq!(set1, set1);
        assert_ne!(set1, set2);
        let union = set1.union(&set2);
        assert!(union.contains(&p1));
        assert!(union.contains(&p2));
        assert!(union.contains(&p3));
        assert!(!union.contains(&p4));
    }

    #[test]
    fn set_intersection() {
        let gc = GameConstants::build(2).unwrap();
        let p1 = Position::build(&gc, 2, 1, 1).unwrap();
        let p2 = Position::build(&gc, 1, 2, 1).unwrap();
        let p3 = Position::build(&gc, 1, 1, 1).unwrap();
        let mut set1 = PositionSet::new();
        let mut set2 = PositionSet::new();
        set1.insert(&p1);
        set1.insert(&p2);
        set2.insert(&p2);
        set2.insert(&p3);
        let intersection = set1.intersection(&set2);
        assert!(!intersection.contains(&p1));
        assert!(intersection.contains(&p2));
        assert!(!intersection.contains(&p3));
    }

    #[test]
    fn set_difference() {
        let gc = GameConstants::build(2).unwrap();
        let p1 = Position::build(&gc, 2, 1, 1).unwrap();
        let p2 = Position::build(&gc, 1, 2, 1).unwrap();
        let p3 = Position::build(&gc, 1, 1, 1).unwrap();
        let mut set1 = PositionSet::new();
        let mut set2 = PositionSet::new();
        set1.insert(&p1);
        set1.insert(&p2);
        set2.insert(&p2);
        set2.insert(&p3);
        let difference = set1.difference(&set2);
        assert!(difference.contains(&p1));
        assert!(!difference.contains(&p2));
        assert!(!difference.contains(&p3));
    }
}
