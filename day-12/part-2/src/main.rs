use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector3(i64, i64, i64);

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[derive(Clone)]
struct NBodySim {
    bodies: Vec<(Vector3, Vector3)>,
}

fn accelerate(pos: &Vector3, other_pos: &Vector3) -> Vector3 {
    Vector3(
        match pos.0.cmp(&other_pos.0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        },
        match pos.1.cmp(&other_pos.1) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        },
        match pos.2.cmp(&other_pos.2) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        },
    )
}

impl Iterator for NBodySim {
    type Item = Vec<(Vector3, Vector3)>;

    fn next(&mut self) -> Option<Vec<(Vector3, Vector3)>> {
        self.bodies = self
            .bodies
            .iter()
            .enumerate()
            .map(|(index, (pos, vel))| {
                let new_vel = self
                    .bodies
                    .iter()
                    .enumerate()
                    .filter(|(index2, _)| index != *index2)
                    .map(|(_, (other_pos, _))| accelerate(pos, other_pos))
                    .fold(vel.clone(), |acc, val| acc + val);
                (*pos + new_vel, new_vel)
            })
            .collect();
        Some(self.bodies.clone())
    }
}

fn loops(mut simulation: NBodySim) -> usize {
    let start_bodies = simulation.bodies.clone();

    let mut iter = 0;

    loop {
        iter += 1;
        if let Some(bodies) = simulation.next() {
            if bodies == start_bodies {
                break;
            }
        } else {
            panic!()
        }
    }

    iter
}

fn main() {
    let simulation = NBodySim {
        bodies: vec![
            (Vector3(-5, 6, -11), Vector3(0, 0, 0)),
            (Vector3(-8, -4, -2), Vector3(0, 0, 0)),
            (Vector3(1, 16, 4), Vector3(0, 0, 0)),
            (Vector3(11, 11, -4), Vector3(0, 0, 0)),
        ],
    };

    let mut simulation_x = simulation.clone();
    for (pos, vel) in simulation_x.bodies.iter_mut() {
        pos.1 = 0;
        pos.2 = 0;
        vel.1 = 0;
        vel.2 = 0;
    }
    let iter_x = loops(simulation_x);

    let mut simulation_y = simulation.clone();
    for (pos, vel) in simulation_y.bodies.iter_mut() {
        pos.0 = 0;
        pos.2 = 0;
        vel.0 = 0;
        vel.2 = 0;
    }
    let iter_y = loops(simulation_y);

    let mut simulation_z = simulation;
    for (pos, vel) in simulation_z.bodies.iter_mut() {
        pos.0 = 0;
        pos.1 = 0;
        vel.0 = 0;
        vel.1 = 0;
    }
    let iter_z = loops(simulation_z);
    println!("steps: lcm({}, {}, {})", iter_x, iter_y, iter_z);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut simulation = NBodySim {
            bodies: vec![
                (Vector3(-1, 0, 2), Vector3(0, 0, 0)),
                (Vector3(2, -10, -7), Vector3(0, 0, 0)),
                (Vector3(4, -8, 8), Vector3(0, 0, 0)),
                (Vector3(3, 5, -1), Vector3(0, 0, 0)),
            ],
        };

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(2, -1, 1), Vector3(3, -1, -1)),
                (Vector3(3, -7, -4), Vector3(1, 3, 3)),
                (Vector3(1, -7, 5), Vector3(-3, 1, -3)),
                (Vector3(2, 2, 0), Vector3(-1, -3, 1)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(5, -3, -1), Vector3(3, -2, -2)),
                (Vector3(1, -2, 2), Vector3(-2, 5, 6)),
                (Vector3(1, -4, -1), Vector3(0, 3, -6)),
                (Vector3(1, -4, 2), Vector3(-1, -6, 2)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(5, -6, -1), Vector3(0, -3, 0)),
                (Vector3(0, 0, 6), Vector3(-1, 2, 4)),
                (Vector3(2, 1, -5), Vector3(1, 5, -4)),
                (Vector3(1, -8, 2), Vector3(0, -4, 0)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(2, -8, 0), Vector3(-3, -2, 1)),
                (Vector3(2, 1, 7), Vector3(2, 1, 1)),
                (Vector3(2, 3, -6), Vector3(0, 2, -1)),
                (Vector3(2, -9, 1), Vector3(1, -1, -1)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(-1, -9, 2), Vector3(-3, -1, 2)),
                (Vector3(4, 1, 5), Vector3(2, 0, -2)),
                (Vector3(2, 2, -4), Vector3(0, -1, 2)),
                (Vector3(3, -7, -1), Vector3(1, 2, -2)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(-1, -7, 3), Vector3(0, 2, 1)),
                (Vector3(3, 0, 0), Vector3(-1, -1, -5)),
                (Vector3(3, -2, 1), Vector3(1, -4, 5)),
                (Vector3(3, -4, -2), Vector3(0, 3, -1)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(2, -2, 1), Vector3(3, 5, -2)),
                (Vector3(1, -4, -4), Vector3(-2, -4, -4)),
                (Vector3(3, -7, 5), Vector3(0, -5, 4)),
                (Vector3(2, 0, 0), Vector3(-1, 4, 2)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(5, 2, -2), Vector3(3, 4, -3)),
                (Vector3(2, -7, -5), Vector3(1, -3, -1)),
                (Vector3(0, -9, 6), Vector3(-3, -2, 1)),
                (Vector3(1, 1, 3), Vector3(-1, 1, 3)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(5, 3, -4), Vector3(0, 1, -2)),
                (Vector3(2, -9, -3), Vector3(0, -2, 2)),
                (Vector3(0, -8, 4), Vector3(0, 1, -2)),
                (Vector3(1, 1, 5), Vector3(0, 0, 2)),
            ))
        );

        assert_eq!(
            simulation.next(),
            Some(vec!(
                (Vector3(2, 1, -3), Vector3(-3, -2, 1)),
                (Vector3(1, -8, 0), Vector3(-1, 1, 3)),
                (Vector3(3, -6, 1), Vector3(3, 2, -3)),
                (Vector3(2, 0, 4), Vector3(1, -1, -1)),
            ))
        );
    }
}
