use std::str::FromStr;

use astro_nalgebra::{
    num_traits::{FromPrimitive, Zero},
    BigFloat, ConstCtx,
};
use nalgebra::{ComplexField, RealField, SMatrix, SVector};

type BF256 = BigFloat<ConstCtx<256>>;

fn main() {
    let input: &str = include_str!("../data/24.txt");
    println!(
        "Answer to part1: {}",
        part1(input, 200000000000000., 400000000000000.)
    );
    println!(
        "Answer to part2: {}",
        part2(input, 200000000000000., 10000000000000.)
    );
}

fn num_list<T>(string: &str) -> Vec<T>
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    string
        .split(", ")
        .map(|num| num.trim().parse().unwrap())
        .collect()
}

fn part1(input: &str, min_dist: f64, max_dist: f64) -> usize {
    let mut rocks: Vec<((f64, f64), (f64, f64))> = vec![];
    for line in input.lines() {
        let pos: Vec<f64> = num_list(line.split(" @ ").nth(0).unwrap());
        let vel: Vec<f64> = num_list(line.split(" @ ").nth(1).unwrap());
        rocks.push(((pos[0], pos[1]), (vel[0], vel[1])));
    }
    let mut intersecting = 0;
    for i in 0..rocks.len() {
        for j in i..rocks.len() {
            // Rock i: x = x0i + vxit0, y = y0i + vyit0
            // Rock j: x = x0j + vxjt1, y = y0j + vyjt1
            // Intersect
            //      x0i + vxit0 = x0j + vxjt1
            //      y0i + vyit0 = y0j + vyjt1
            //      t0 = (x0j - x0i + vxjt1) / vxi
            //      t0 = (y0j - y0i + vyjt1) / vyi
            //      vxi(y0j-y0i + vyjt1) = vyi(x0j-x0i + vxjt1)
            //      t1 = (vxi * (y0i - y0j) + xyi * (x0j-x0i)) / (vxi*vyj - vui*vxj)
            let (x0i, y0i) = rocks[i].0;
            let (vxi, vyi) = rocks[i].1;
            let (x0j, y0j) = rocks[j].0;
            let (vxj, vyj) = rocks[j].1;
            let denom = vxi * vyj - vyi * vxj;
            if denom == 0. {
                continue;
            }
            let t1 = (vxi * (y0i - y0j) + vyi * (x0j - x0i)) / denom;
            let x_inter = x0j + vxj * t1;
            let y_inter = y0j + vyj * t1;
            let t0 = if vxi == 0. {
                (x0j - x0i + vxj * t1) / vxi
            } else {
                (y0j - y0i + vyj * t1) / vyi
            };
            if t1 < 0. {
                continue;
            }
            if t0 < 0. {
                continue;
            }
            if x_inter < min_dist || x_inter > max_dist {
                continue;
            }
            if y_inter < min_dist || y_inter > max_dist {
                continue;
            }
            intersecting += 1;
        }
    }
    intersecting
}
fn part2(input: &str, center: f64, scale: f64) -> usize {
    let center: BF256 = BF256::from_f64(center).unwrap();
    let scale = BF256::from_f64(scale).unwrap();
    // Okay so I ran into some limitations with floating point numbers and this solution
    // is not ideal. Basically I used Newton's multivariable method to approximate the values of the
    // starting point, the starting velocity, and the times it intersected with 3 of the rocks.
    // The type f64 is not accurate enough for this problem, and the equation solver I used did not
    // support anything more accurate than that, so I had to rewrite my own version of the solver
    // and use a package that I wrote to add arbitrary precision float bindings to nalgebra
    // But it works and it's my own solution!!
    let mut rocks: Vec<((BF256, BF256, BF256), (BF256, BF256, BF256))> = vec![];
    for line in input.lines() {
        let pos: Vec<BF256> = num_list(line.split(" @ ").nth(0).unwrap());
        let vel: Vec<BF256> = num_list(line.split(" @ ").nth(1).unwrap());
        rocks.push((
            (
                (pos[0].clone() - center.clone()) / scale.clone(),
                (pos[1].clone() - center.clone()) / scale.clone(),
                (pos[2].clone() - center.clone()) / scale.clone(),
            ),
            (vel[0].clone(), vel[1].clone(), vel[2].clone()),
        ));
    }

    // Rock can be determined by 3 rock collisions because each collision adds 1 unknown and 3
    // equations and we start with 6 unknowns.
    // So with 3 rocks there are 9 equations and 9 unknowns (exact answer)

    // v is stored as [x,y,z,vx,vy,vz,t1,t2,t3]
    let func = |v: SVector<BF256, 9>| {
        #[rustfmt::skip]
        return SVector::<BF256, 9>::from([
            v[0].clone() + v[3].clone() * v[6].clone() - rocks[0].0 .0.clone() - rocks[0].1 .0.clone() * v[6].clone(),
            v[1].clone() + v[4].clone() * v[6].clone() - rocks[0].0 .1.clone() - rocks[0].1 .1.clone() * v[6].clone(),
            v[2].clone() + v[5].clone() * v[6].clone() - rocks[0].0 .2.clone() - rocks[0].1 .2.clone() * v[6].clone(),
            v[0].clone() + v[3].clone() * v[7].clone() - rocks[1].0 .0.clone() - rocks[1].1 .0.clone() * v[7].clone(),
            v[1].clone() + v[4].clone() * v[7].clone() - rocks[1].0 .1.clone() - rocks[1].1 .1.clone() * v[7].clone(),
            v[2].clone() + v[5].clone() * v[7].clone() - rocks[1].0 .2.clone() - rocks[1].1 .2.clone() * v[7].clone(),
            v[0].clone() + v[3].clone() * v[8].clone() - rocks[2].0 .0.clone() - rocks[2].1 .0.clone() * v[8].clone(),
            v[1].clone() + v[4].clone() * v[8].clone() - rocks[2].0 .1.clone() - rocks[2].1 .1.clone() * v[8].clone(),
            v[2].clone() + v[5].clone() * v[8].clone() - rocks[2].0 .2.clone() - rocks[2].1 .2.clone() * v[8].clone(),
        ]);
    };
    let z = BF256::zero();
    let o: BF256 = "1".parse().unwrap();
    let jac = |v: SVector<BF256, 9>| {
        #[rustfmt::skip]
        let mut mat = SMatrix::<BF256, 9, 9>::from([
            [ o.clone(), z.clone(), z.clone(), v[6].clone(), z.clone(), z.clone(), v[3].clone() - rocks[0].1 .0.clone(), z.clone(), z.clone(), ],
            [ z.clone(), o.clone(), z.clone(), z.clone(), v[6].clone(), z.clone(), v[4].clone() - rocks[0].1 .1.clone(), z.clone(), z.clone(), ],
            [ z.clone(), z.clone(), o.clone(), z.clone(), z.clone(), v[6].clone(), v[5].clone() - rocks[0].1 .2.clone(), z.clone(), z.clone(), ],
            [ o.clone(), z.clone(), z.clone(), v[7].clone(), z.clone(), z.clone(), z.clone(), v[3].clone() - rocks[1].1 .0.clone(), z.clone(), ],
            [ z.clone(), o.clone(), z.clone(), z.clone(), v[7].clone(), z.clone(), z.clone(), v[4].clone() - rocks[1].1 .1.clone(), z.clone(), ],
            [ z.clone(), z.clone(), o.clone(), z.clone(), z.clone(), v[7].clone(), z.clone(), v[5].clone() - rocks[1].1 .2.clone(), z.clone(), ],
            [ o.clone(), z.clone(), z.clone(), v[8].clone(), z.clone(), z.clone(), z.clone(), z.clone(), v[3].clone() - rocks[2].1 .0.clone(), ],
            [ z.clone(), o.clone(), z.clone(), z.clone(), v[8].clone(), z.clone(), z.clone(), z.clone(), v[4].clone() - rocks[2].1 .1.clone(), ],
            [ z.clone(), z.clone(), o.clone(), z.clone(), z.clone(), v[8].clone(), z.clone(), z.clone(), v[5].clone() - rocks[2].1 .2.clone(), ],
        ]);
        mat.transpose_mut();
        mat
    };
    let solution = solve(
        // Starting vector
        SVector::<BF256, 9>::from([
            "1".parse().unwrap(),
            "9".parse().unwrap(),
            "3".parse().unwrap(),
            "100".parse().unwrap(),
            "101".parse().unwrap(),
            "102".parse().unwrap(),
            "1".parse().unwrap(),
            "4".parse().unwrap(),
            "3".parse().unwrap(),
        ]),
        func,
        jac,
        100,
        "0.00001".parse().unwrap(),
    )
    .unwrap();
    ((solution[0].clone() + solution[1].clone() + solution[2].clone()) * scale.clone())
        .round()
        .as_int()
        .unwrap()
        .1 as usize
}

/// Solver from https://docs.rs/crate/eqsolver/latest
/// Although I had to modify it because it required Copy and BigFloat doesn't implement that
pub fn solve<T, const D: usize>(
    mut x0: SVector<T, D>,
    mut function: impl FnMut(SVector<T, D>) -> SVector<T, D>,
    mut jacobian: impl FnMut(SVector<T, D>) -> SMatrix<T, D, D>,
    iter_max: usize,
    tolerance: T,
) -> Option<SVector<T, D>>
where
    T: ComplexField + RealField,
{
    let mut dv = x0.clone().add_scalar(T::max_value()?); // We assume error vector is infinitely long at the start
    let mut iter = 1;

    // Newton-Raphson Iteration
    while dv.abs().max() > tolerance && iter < iter_max {
        if let Some(j_inv) = (jacobian)(x0.clone()).try_inverse() {
            dv = j_inv * (function)(x0.clone());
            x0 = x0 - dv.clone();
            iter += 1;
            println!("{x0}");
        } else {
            return None;
        }
    }
    if iter >= iter_max {
        return None;
    }
    Some(x0)
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    assert_eq!(part1(input, 7., 27.), 2);
    assert_eq!(part2(input, 0., 1.), 47);
}
