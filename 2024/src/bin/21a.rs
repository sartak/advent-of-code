use anyhow::{Result, bail};
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Numpad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Arrowpad {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl TryFrom<char> for Numpad {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Numpad::*;
        Ok(match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'A' => Activate,
            _ => bail!("Invalid char '{c}'"),
        })
    }
}

impl Numpad {
    fn keys() -> Vec<Numpad> {
        use Numpad::*;
        vec![
            Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Activate,
        ]
    }

    fn complexity(keys: &[Self]) -> usize {
        let mut answer = 0;
        for key in keys {
            use Numpad::*;
            let digit = match key {
                Zero => 0,
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Activate => continue,
            };
            answer = answer * 10 + digit;
        }
        answer
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        use Direction::*;
        use Numpad::*;

        match self {
            Zero => match direction {
                Up => Some(Two),
                Down => None,
                Left => None,
                Right => Some(Activate),
            },
            One => match direction {
                Up => Some(Four),
                Down => None,
                Left => None,
                Right => Some(Two),
            },
            Two => match direction {
                Up => Some(Five),
                Down => Some(Zero),
                Left => Some(One),
                Right => Some(Three),
            },
            Three => match direction {
                Up => Some(Six),
                Down => Some(Activate),
                Left => Some(Two),
                Right => None,
            },
            Four => match direction {
                Up => Some(Seven),
                Down => Some(One),
                Left => None,
                Right => Some(Five),
            },
            Five => match direction {
                Up => Some(Eight),
                Down => Some(Two),
                Left => Some(Four),
                Right => Some(Six),
            },
            Six => match direction {
                Up => Some(Nine),
                Down => Some(Three),
                Left => Some(Five),
                Right => None,
            },
            Seven => match direction {
                Up => None,
                Down => Some(Four),
                Left => None,
                Right => Some(Eight),
            },
            Eight => match direction {
                Up => None,
                Down => Some(Five),
                Left => Some(Seven),
                Right => Some(Nine),
            },
            Nine => match direction {
                Up => None,
                Down => Some(Six),
                Left => Some(Eight),
                Right => None,
            },
            Activate => match direction {
                Up => Some(Three),
                Down => None,
                Left => Some(Zero),
                Right => None,
            },
        }
    }
}

impl Arrowpad {
    fn keys() -> Vec<Arrowpad> {
        use Arrowpad::*;
        vec![Up, Down, Left, Right, Activate]
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        use Arrowpad::*;

        match self {
            Up => match direction {
                Direction::Up => None,
                Direction::Down => Some(Down),
                Direction::Left => None,
                Direction::Right => Some(Activate),
            },
            Down => match direction {
                Direction::Up => Some(Up),
                Direction::Down => None,
                Direction::Left => Some(Left),
                Direction::Right => Some(Right),
            },
            Left => match direction {
                Direction::Up => None,
                Direction::Down => None,
                Direction::Left => None,
                Direction::Right => Some(Down),
            },
            Right => match direction {
                Direction::Up => Some(Activate),
                Direction::Down => None,
                Direction::Left => Some(Down),
                Direction::Right => None,
            },
            Activate => match direction {
                Direction::Up => None,
                Direction::Down => Some(Right),
                Direction::Left => Some(Up),
                Direction::Right => None,
            },
        }
    }
}

impl Direction {
    fn all() -> Vec<Self> {
        use Direction::*;
        vec![Up, Down, Left, Right]
    }

    fn reverse(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn to_arrow(&self) -> Arrowpad {
        match self {
            Direction::Up => Arrowpad::Up,
            Direction::Down => Arrowpad::Down,
            Direction::Left => Arrowpad::Left,
            Direction::Right => Arrowpad::Right,
        }
    }
}

impl Display for Numpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Numpad::*;
        write!(
            f,
            "{}",
            match self {
                Zero => '0',
                One => '1',
                Two => '2',
                Three => '3',
                Four => '4',
                Five => '5',
                Six => '6',
                Seven => '7',
                Eight => '8',
                Nine => '9',
                Activate => 'A',
            }
        )
    }
}

impl Display for Arrowpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Arrowpad::*;
        write!(
            f,
            "{}",
            match self {
                Up => '^',
                Down => 'v',
                Left => '<',
                Right => '>',
                Activate => 'A',
            }
        )
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Direction::*;
        write!(
            f,
            "{}",
            match self {
                Up => '^',
                Down => 'v',
                Left => '<',
                Right => '>',
            }
        )
    }
}

fn numpad_maneuvers(origin: Numpad, destination: Numpad) -> Vec<Vec<Direction>> {
    if origin == destination {
        return vec![vec![]];
    }

    for steps in 0.. {
        let mut queue = vec![(origin, Vec::new())];
        for _ in 0..steps {
            let mut newqueue = Vec::new();
            let mut result = Vec::new();
            for (from, path) in queue {
                for dir in Direction::all() {
                    let Some(to) = from.step(dir) else {
                        continue;
                    };
                    let mut path = path.clone();
                    path.push(dir);
                    if to == destination {
                        result.push(path.clone());
                    }
                    newqueue.push((to, path));
                }
            }
            queue = newqueue;

            if !result.is_empty() {
                return result;
            }
        }
    }

    unreachable!();
}

fn arrowpad_maneuvers(origin: Arrowpad, destination: Arrowpad) -> Vec<Vec<Direction>> {
    if origin == destination {
        return vec![vec![]];
    }

    for steps in 0.. {
        let mut queue = vec![(origin, Vec::new())];
        for _ in 0..steps {
            let mut newqueue = Vec::new();
            let mut result = Vec::new();
            for (from, path) in queue {
                for dir in Direction::all() {
                    let Some(to) = from.step(dir) else {
                        continue;
                    };
                    let mut path = path.clone();
                    path.push(dir);
                    if to == destination {
                        result.push(path.clone());
                    }
                    newqueue.push((to, path));
                }
            }
            queue = newqueue;

            if !result.is_empty() {
                return result;
            }
        }
    }

    unreachable!();
}

fn all_numpad_maneuvers() -> HashMap<(Numpad, Numpad), Vec<Vec<Direction>>> {
    let mut maneuvers = HashMap::new();

    for from in Numpad::keys() {
        for to in Numpad::keys() {
            let paths = numpad_maneuvers(from, to);
            maneuvers.insert((from, to), paths);
        }
    }

    maneuvers
}

fn all_arrowpad_maneuvers() -> HashMap<(Arrowpad, Arrowpad), Vec<Vec<Direction>>> {
    let mut maneuvers = HashMap::new();

    for from in Arrowpad::keys() {
        for to in Arrowpad::keys() {
            let paths = arrowpad_maneuvers(from, to);
            maneuvers.insert((from, to), paths);
        }
    }

    maneuvers
}

fn directions_for_nums(nums: &[Numpad]) -> Vec<Vec<Vec<Direction>>> {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, Numpad::Activate, Vec::new()));

    let maneuvers = all_numpad_maneuvers();
    let mut best = None;

    while let Some((score, typed, current, path)) = queue.pop() {
        let score = score.0;

        if let Some((b, _)) = best {
            if score > b {
                continue;
            }
        }

        if typed == nums.len() {
            if let Some((b, ref mut paths)) = best {
                if score < b {
                    best = Some((score, vec![path]));
                } else if score == b {
                    paths.push(path);
                }
            } else {
                best = Some((score, vec![path]));
            }

            continue;
        }

        let next = nums[typed];
        let paths = maneuvers.get(&(current, next)).unwrap();
        for p in paths {
            let mut path = path.clone();
            path.push(p.clone());
            queue.push((Reverse(score + p.len()), typed + 1, next, path));
        }
    }

    best.unwrap().1
}

fn directions_for_arrows(arrows: &[Arrowpad]) -> Vec<Vec<Vec<Direction>>> {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, Arrowpad::Activate, Vec::new()));

    let maneuvers = all_arrowpad_maneuvers();

    let mut best = None;

    while let Some((score, typed, current, path)) = queue.pop() {
        let score = score.0;

        if let Some((b, _)) = best {
            if score > b {
                continue;
            }
        }

        if typed == arrows.len() {
            if let Some((b, ref mut paths)) = best {
                if score < b {
                    best = Some((score, vec![path]));
                } else {
                    paths.push(path);
                }
            } else {
                best = Some((score, vec![path]));
            }

            continue;
        }

        let next = arrows[typed];
        let paths = maneuvers.get(&(current, next)).unwrap();
        for p in paths {
            let mut path = path.clone();
            path.push(p.clone());
            queue.push((Reverse(score + p.len()), typed + 1, next, path));
        }
    }

    best.unwrap().1
}

fn directions_to_arrows(dirs: &[Vec<Direction>]) -> Vec<Arrowpad> {
    let mut result = Vec::new();
    for path in dirs {
        for key in path {
            result.push(key.to_arrow());
        }
        result.push(Arrowpad::Activate);
    }
    result
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/21-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/21.txt")?;

    let mut answer = 0;

    for line in input.lines() {
        let numbers = line
            .chars()
            .map(|n| Numpad::try_from(n).unwrap())
            .collect_vec();
        let complexity = Numpad::complexity(&numbers);

        let mut dirs = directions_for_nums(&numbers);

        for _ in 0..2 {
            dirs = dirs
                .iter()
                .unique()
                .map(|d| directions_to_arrows(d))
                .unique()
                .flat_map(|a| directions_for_arrows(&a))
                .collect_vec();
        }

        let arrows = dirs
            .iter()
            .unique()
            .map(|d| directions_to_arrows(d))
            .collect_vec();

        let mut shortest = arrows[0].len();
        for arrow in &arrows {
            if arrow.len() < shortest {
                shortest = arrow.len();
            }
        }
        eprintln!("{line}: {shortest} * {complexity}");
        answer += shortest * complexity;
    }

    println!("{answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numpad() {
        for key in Numpad::keys() {
            for first in Direction::all() {
                if let Some(other) = key.step(first) {
                    assert_eq!(key, other.step(first.reverse()).unwrap());
                }
            }
        }
    }

    #[test]
    fn test_arrowpad() {
        for key in Arrowpad::keys() {
            for first in Direction::all() {
                if let Some(other) = key.step(first) {
                    assert_eq!(key, other.step(first.reverse()).unwrap());
                }
            }
        }
    }

    #[test]
    fn test_numpad_maneuvers() {
        let empty = numpad_maneuvers(Numpad::Six, Numpad::Six);
        assert_eq!(empty.len(), 1);
        assert_eq!(empty[0].len(), 0);

        let one = numpad_maneuvers(Numpad::Six, Numpad::Three);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].len(), 1);
        assert_eq!(one[0][0], Direction::Down);
    }
}
