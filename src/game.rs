use std::collections::VecDeque;

use rand::seq::IndexedRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn step(self, direction: Direction) -> Point {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Playing,
    GameOver,
    Won,
}

pub struct Game {
    width: i32,
    height: i32,
    /// Front is the head, back is the tail.
    snake: VecDeque<Point>,
    direction: Direction,
    next_direction: Direction,
    food: Option<Point>,
    score: u32,
    status: Status,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        assert!(width >= 4 && height >= 1, "board must be at least 4x1");
        let head = Point {
            x: width / 2,
            y: height / 2,
        };
        let snake = VecDeque::from([
            head,
            Point {
                x: head.x - 1,
                ..head
            },
            Point {
                x: head.x - 2,
                ..head
            },
        ]);
        let mut game = Game {
            width,
            height,
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: None,
            score: 0,
            status: Status::Playing,
        };
        game.place_food();
        game
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn snake(&self) -> impl Iterator<Item = Point> + '_ {
        self.snake.iter().copied()
    }

    pub fn head(&self) -> Point {
        self.snake[0]
    }

    pub fn food(&self) -> Option<Point> {
        self.food
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn status(&self) -> Status {
        self.status
    }

    /// Takes effect on the next tick. Compared against the direction actually
    /// moved, so two quick presses can't fold the snake back onto itself.
    pub fn turn(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.next_direction = direction;
        }
    }

    pub fn tick(&mut self) {
        if self.status != Status::Playing {
            return;
        }

        self.direction = self.next_direction;
        let new_head = self.head().step(self.direction);

        if !self.in_bounds(new_head) {
            self.status = Status::GameOver;
            return;
        }

        let grows = self.food == Some(new_head);
        // The tail moves out of the way this tick unless the snake is growing.
        let blocking = if grows {
            self.snake.len()
        } else {
            self.snake.len() - 1
        };
        if self.snake.iter().take(blocking).any(|&p| p == new_head) {
            self.status = Status::GameOver;
            return;
        }

        self.snake.push_front(new_head);
        if grows {
            self.score += 1;
            self.place_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn in_bounds(&self, p: Point) -> bool {
        (0..self.width).contains(&p.x) && (0..self.height).contains(&p.y)
    }

    fn place_food(&mut self) {
        let free: Vec<Point> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Point { x, y }))
            .filter(|p| !self.snake.contains(p))
            .collect();
        self.food = free.choose(&mut rand::rng()).copied();
        if self.food.is_none() {
            self.status = Status::Won;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    /// A 10x10 game with food parked out of the way in the bottom-right corner.
    fn game() -> Game {
        let mut game = Game::new(10, 10);
        game.food = Some(p(9, 9));
        game
    }

    #[test]
    fn starts_in_the_middle_heading_right() {
        let game = game();
        assert_eq!(
            game.snake().collect::<Vec<_>>(),
            [p(5, 5), p(4, 5), p(3, 5)]
        );
        assert_eq!(game.status(), Status::Playing);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn food_never_spawns_on_the_snake() {
        for _ in 0..100 {
            let game = Game::new(4, 1);
            assert_eq!(game.food(), Some(p(3, 0)));
        }
    }

    #[test]
    fn tick_moves_forward_without_growing() {
        let mut game = game();
        game.tick();
        assert_eq!(
            game.snake().collect::<Vec<_>>(),
            [p(6, 5), p(5, 5), p(4, 5)]
        );
    }

    #[test]
    fn turn_applies_on_next_tick() {
        let mut game = game();
        game.turn(Direction::Up);
        assert_eq!(game.head(), p(5, 5));
        game.tick();
        assert_eq!(game.head(), p(5, 4));
    }

    #[test]
    fn cannot_reverse_into_itself() {
        let mut game = game();
        game.turn(Direction::Left);
        game.tick();
        assert_eq!(game.head(), p(6, 5));
    }

    #[test]
    fn two_quick_turns_cannot_reverse() {
        let mut game = game();
        game.turn(Direction::Up);
        game.turn(Direction::Left);
        game.tick();
        assert_eq!(game.head(), p(5, 4));
        assert_eq!(game.status(), Status::Playing);
    }

    #[test]
    fn eating_food_grows_and_scores() {
        let mut game = game();
        game.food = Some(p(6, 5));
        game.tick();
        assert_eq!(game.snake().count(), 4);
        assert_eq!(game.score(), 1);
        assert_ne!(game.food(), Some(p(6, 5)));
    }

    #[test]
    fn hitting_a_wall_ends_the_game() {
        let mut game = game();
        for _ in 0..4 {
            game.tick();
        }
        assert_eq!(game.head(), p(9, 5));
        assert_eq!(game.status(), Status::Playing);
        game.tick();
        assert_eq!(game.status(), Status::GameOver);
    }

    #[test]
    fn hitting_own_body_ends_the_game() {
        let mut game = game();
        // Head at (5,5) moving up, body wrapping round so (5,4) is occupied.
        game.snake = VecDeque::from([p(5, 5), p(6, 5), p(6, 4), p(5, 4), p(4, 4), p(4, 5)]);
        game.direction = Direction::Left;
        game.turn(Direction::Up);
        game.tick();
        assert_eq!(game.status(), Status::GameOver);
    }

    #[test]
    fn can_follow_own_tail() {
        let mut game = game();
        // A 2x2 loop: the head moves into the cell the tail is leaving.
        game.snake = VecDeque::from([p(5, 5), p(6, 5), p(6, 4), p(5, 4)]);
        game.direction = Direction::Left;
        game.turn(Direction::Up);
        game.tick();
        assert_eq!(game.status(), Status::Playing);
        assert_eq!(game.head(), p(5, 4));
    }

    #[test]
    fn nothing_happens_after_game_over() {
        let mut game = game();
        game.status = Status::GameOver;
        game.tick();
        assert_eq!(game.head(), p(5, 5));
    }

    #[test]
    fn filling_the_board_wins() {
        let mut game = Game::new(4, 1);
        game.tick();
        assert_eq!(game.score(), 1);
        assert_eq!(game.status(), Status::Won);
        assert_eq!(game.food(), None);
    }
}
