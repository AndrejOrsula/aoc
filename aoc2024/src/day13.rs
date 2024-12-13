use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
#[must_use]
pub fn part1(input: &str) -> i64 {
    Problem::parse(input).solve()
}

#[aoc(day13, part2)]
#[must_use]
pub fn part2(input: &str) -> i64 {
    let mut problem = Problem::parse(input);
    problem.iter_mut().for_each(|behaviour| {
        behaviour.goal += nalgebra::Vector2::from_element(10_000_000_000_000);
    });
    problem.solve()
}

#[derive(derive_more::Deref, derive_more::DerefMut)]
struct Problem(Vec<Behaviour>);

struct Behaviour {
    a_button: nalgebra::Vector2<i64>,
    b_button: nalgebra::Vector2<i64>,
    goal: nalgebra::Vector2<i64>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        Self(
            input
                .split("\n\n")
                .map(|x| {
                    let mut lines = x.split('\n');
                    let a = Self::parse_inner(lines.next().unwrap());
                    let b = Self::parse_inner(lines.next().unwrap());

                    let (_, prize) = lines.next().unwrap().rsplit_once(": ").unwrap();
                    let (x, y) = prize.split_once(", ").unwrap();
                    let prize = (x[2..].parse().unwrap(), y[2..].parse().unwrap());

                    Behaviour {
                        a_button: nalgebra::Vector2::new(a.0, a.1),
                        b_button: nalgebra::Vector2::new(b.0, b.1),
                        goal: nalgebra::Vector2::new(prize.0, prize.1),
                    }
                })
                .collect(),
        )
    }

    fn parse_inner(line: &str) -> (i64, i64) {
        let (_, parts) = line.rsplit_once(": ").unwrap();
        let (x, y) = parts.split_once(", ").unwrap();
        (x[1..].parse().unwrap(), y[1..].parse().unwrap())
    }

    fn solve(&self) -> i64 {
        self.iter()
            .map(|behaviour| {
                let a = (behaviour.b_button.y * behaviour.goal.x
                    - behaviour.b_button.x * behaviour.goal.y)
                    / (behaviour.a_button.x * behaviour.b_button.y
                        - behaviour.a_button.y * behaviour.b_button.x);
                let b = (behaviour.goal.x - behaviour.a_button.x * a) / behaviour.b_button.x;

                if a <= 0
                    || b <= 0
                    || behaviour.goal != behaviour.a_button * a + behaviour.b_button * b
                {
                    0
                } else {
                    3 * a + b
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 480);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 875_318_608_908);
    }
}
