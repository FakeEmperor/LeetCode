/*
 * @lc app=leetcode id=735 lang=rust
 *
 * [735] Asteroid Collision
 */

pub struct Solution;


// @lc code=start
impl Solution {

    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut positive = vec![];
        let mut answer = vec![];
        for asteroid in asteroids {
            if asteroid > 0 {
                positive.push(asteroid);
            } else {
                while positive.len() > 0 {
                    if -asteroid > positive[positive.len() - 1] {
                        positive.pop();
                    } else {
                        break;
                    }
                }
                if positive.len() == 0 {
                    answer.push(asteroid);
                } else if -asteroid == positive[positive.len() - 1] {
                    positive.pop();
                }
            }
        }
        answer.append(&mut positive);
        return answer;
    }
}
// @lc code=end

