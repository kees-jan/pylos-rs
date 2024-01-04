type PylosResult<T> = Result<T, String>;

fn total_balls_for_layer_count(layers: u32) -> u32 {
    layers * (layers + 1) * (2 * layers + 1) / 6
}

struct GameConstants {
    layers: u32,
}

impl GameConstants {
    fn build(layers: u32) -> PylosResult<GameConstants> {
        let total_balls = total_balls_for_layer_count(layers);
        if total_balls > 64 {
            return Err(format!("A game with {layers} layers needs {total_balls} balls, which is more than supported"));
        }

        Ok(GameConstants { layers })
    }
}
