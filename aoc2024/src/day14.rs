use aoc_runner_derive::aoc;

#[aoc(day14, part1)]
#[must_use]
pub fn part1(input: &str) -> i32 {
    let mut problem = Environment::parse(input);
    problem
        .robots
        .iter_mut()
        .for_each(|x| x.step(problem.limits, 100));
    problem.safety_factor()
}

#[aoc(day14, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut problem = Environment::parse(input);
    for i in 0.. {
        problem
            .robots
            .iter_mut()
            .for_each(|x| x.step(problem.limits, 1));
        if problem
            .robots
            .iter()
            .map(|x| (x.pos - problem.limits / 2).abs().sum())
            .sum::<i32>()
            < 2_i32.pow(14)
        {
            return i + 1;
        }
    }
    unreachable!()
}

struct Environment {
    robots: smallvec::SmallVec<[Robot; 512]>,
    limits: nalgebra::Vector2<i32>,
}

impl Environment {
    fn parse(input: &str) -> Self {
        let robots: smallvec::SmallVec<[Robot; 512]> = input.lines().map(Robot::parse).collect();
        let limits = robots
            .iter()
            .fold(nalgebra::Vector2::new(0, 0), |mut limits, robot| {
                limits.x = limits.x.max(robot.pos.x);
                limits.y = limits.y.max(robot.pos.y);
                limits
            })
            + nalgebra::Vector2::new(1, 1);
        Self { robots, limits }
    }

    fn safety_factor(&self) -> i32 {
        let mut quad = [0; 4];
        for pos in self.robots.iter().map(|robot| robot.pos) {
            if pos.x == (self.limits / 2).x || pos.y == (self.limits / 2).y {
                continue;
            }
            quad[(2 * usize::from((0..=(self.limits / 2).x).contains(&pos.x)))
                | usize::from((0..=(self.limits / 2).y).contains(&pos.y))] += 1;
        }
        quad.iter().product()
    }
}

struct Robot {
    pos: nalgebra::Vector2<i32>,
    vel: nalgebra::Vector2<i32>,
}

impl Robot {
    fn parse(input: &str) -> Self {
        fn parse_coordinates(input: &str) -> nalgebra::Vector2<i32> {
            let (x, y) = input.split_once(',').unwrap();
            nalgebra::Vector2::new(x.parse().unwrap(), y.parse().unwrap())
        }
        let (pos, vel) = input.split_once(' ').unwrap();
        let pos = parse_coordinates(&pos[2..]);
        let vel = parse_coordinates(&vel[2..]);
        Self { pos, vel }
    }

    fn step(&mut self, bounds: nalgebra::Vector2<i32>, dt: i32) {
        self.pos += dt * self.vel;
        self.pos.x = (self.pos.x % bounds.x + bounds.x) % bounds.x;
        self.pos.y = (self.pos.y % bounds.y + bounds.y) % bounds.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 12);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 1);
    }
}
