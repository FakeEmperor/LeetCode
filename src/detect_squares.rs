/*
 * @lc app=leetcode id=2013 lang=rust
 *
 * [2013] Detect Squares
 */

// @lc code=start
use std::{collections::HashMap, ops::AddAssign};

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct DetectSquares {
    pub vertices: HashMap<i32, Vec<Point>>,
    pub points: HashMap<Point, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    pub fn new() -> Self {
        DetectSquares {
            vertices: HashMap::new(),
            points: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let p = Point {
            x: point[0],
            y: point[1],
        };
        match self.points.get_mut(&p) {
            None => {
                self.points.insert(p.clone(), 1);

                match self.vertices.get_mut(&point[0]) {
                    None => {
                        self.vertices.insert(point[0], vec![p.clone()]);
                    }
                    Some(vec) => {
                        vec.push(p.clone());
                    }
                }
            }
            Some(count) => count.add_assign(1),
        }
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let mut total_count: i32 = 0;
        let matched_vertices = self.vertices.get(&point[0]);
        if matched_vertices.is_none() {
            return 0;
        }

        for point_2 in matched_vertices.unwrap().iter() {
            if point_2.y == point[1] {
                continue;
            }
            let delta = (point_2.y - point[1]).abs();
            // searching for left square
            let mut point_3: Point = Point {
                x: point_2.x - delta,
                y: point[1],
            };
            let mut point_4: Point = Point {
                x: point_2.x - delta,
                y: point_2.y,
            };
            total_count += self.points.get(&point_2).unwrap_or(&0)
                * self.points.get(&point_3).unwrap_or(&0)
                * self.points.get(&point_4).unwrap_or(&0);

            point_3 = Point {
                x: point_2.x + delta,
                y: point[1],
            };
            point_4 = Point {
                x: point_2.x + delta,
                y: point_2.y,
            };
            total_count += self.points.get(&point_2).unwrap_or(&0)
                * self.points.get(&point_3).unwrap_or(&0)
                * self.points.get(&point_4).unwrap_or(&0);
        }
        return total_count;
    }
}

// @lc code=end
