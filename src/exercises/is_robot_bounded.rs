use std::fmt::{Debug, Formatter, Result};

// On an infinite plane, a robot initially stands at (0, 0) and faces north.  The robot can receive one of three instructions:
//
// "G": go straight 1 unit;
// "L": turn 90 degrees to the left;
// "R": turn 90 degrees to the right.
// The robot performs the instructions given in order, and repeats them forever.
//
// Return true if and only if there exists a circle in the plane such that the robot never leaves the circle.
struct Solution;

struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn plus(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn rotate_90_cw(&mut self) {
        self.x *= -1;
        let tmp = self.x;
        self.x = self.y;
        self.y = tmp;
    }

    fn rotate_90_ccw(&mut self) {
        self.y *= -1;
        let tmp = self.y;
        self.y = self.x;
        self.x = tmp;
    }
}

const NORTH: Point = Point { x: 0, y: 1 };
const ZERO: Point = Point { x: 0, y: 1 };

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut direction = NORTH;
        let mut pos = ZERO;
        for c in instructions.chars() {
            println!("{}", c);
            match c {
                'G' => {
                    pos.plus(&direction);
                }
                'L' => {
                    direction.rotate_90_ccw();
                }
                'R' => {
                    direction.rotate_90_cw();
                }
                _ => {
                    panic!();
                }
            };
        }
        println!("Final position:{:?}, direction:{:?}", pos, direction);
        pos == ZERO || direction != NORTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_robot_bounded() {
        assert_eq!(Solution::is_robot_bounded(String::from("GGLLGG")), true);
        assert_eq!(Solution::is_robot_bounded(String::from("GG")), false);
        assert_eq!(Solution::is_robot_bounded(String::from("GL")), true);
    }

    #[test]
    fn test_rotate_cw() {
        test_rotate_cw_0(0, 1, 1, 0);
        test_rotate_cw_0(1, 0, 0, -1);
        test_rotate_cw_0(0, -1, -1, 0);
        test_rotate_cw_0(-1, 0, 0, 1);
    }

    fn test_rotate_cw_0(x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut p1 = Point::new(x1, y1);
        p1.rotate_90_cw();
        assert_eq!(p1, Point::new(x2, y2));
    }

    #[test]
    fn test_rotate_ccw() {
        test_rotate_ccw_0(0, 1, -1, 0);
        test_rotate_ccw_0(-1, 0, 0, -1);
        test_rotate_ccw_0(0, -1, 1, 0);
        test_rotate_ccw_0(1, 0, 0, 1);
    }

    fn test_rotate_ccw_0(x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut p1 = Point::new(x1, y1);
        p1.rotate_90_ccw();
        assert_eq!(p1, Point::new(x2, y2));
    }
}