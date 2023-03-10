use rand::{seq::IteratorRandom, thread_rng};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn set_opposite_direction(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}

pub fn select_move(direction_queue: Vec<Direction>, current_direction: Direction) -> Direction {
    let mut rng = thread_rng();
    let opposite_direction = set_opposite_direction(current_direction);

    let chosen_direciton = direction_queue
        .iter()
        .filter(|direction| **direction != opposite_direction)
        .choose(&mut rng);

    match chosen_direciton {
        Some(&dir) => dir,
        None => current_direction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn debug_queue() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
    }
    #[test]
    fn select_move_test() {
        for _ in 0..1000 {
            let current_direction = Direction::Up;
            let selected_move = select_move(debug_queue(), current_direction);
            assert_ne!(selected_move, Direction::Down);
        }
    }
    #[test]
    fn select_move_test_2() {
        let mut debug_iter = vec![
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
        ]
        .into_iter();
        let debug_queue = debug_queue();
        for dir in debug_queue {
            assert_eq!(set_opposite_direction(dir), debug_iter.next().unwrap());
        }
    }
}
