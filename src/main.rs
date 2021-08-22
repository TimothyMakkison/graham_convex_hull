mod graham_hull;
mod point;

use crate::graham_hull::convex_hull;
use crate::point::Point;

fn main() {
    let points = vec![
        Point::new(3.0, 6.0),
        Point::new(5.0, 2.0),
        Point::new(4.0, 4.0),
        Point::new(2.0, 3.0),
        Point::new(1.0, 1.0),
        Point::new(4.0, 3.0),
        Point::new(3.0, 4.0),
        Point::new(0.0, 5.0),
    ];

    let hull = convex_hull(&points);

    println!("{:?}", hull);
    println!("");
    println!("{:?}", points);
}

#[test]
fn returns_hull_with_valid_input() {
    // arrange
    let points = vec![
        Point::new(3.0, 6.0),
        Point::new(5.0, 2.0),
        Point::new(4.0, 4.0),
        Point::new(2.0, 3.0),
        Point::new(1.0, 1.0),
        Point::new(4.0, 3.0),
        Point::new(3.0, 4.0),
        Point::new(0.0, 5.0),
    ];

    let hull_points = vec![
        Point::new(1.0, 1.0),
        Point::new(5.0, 2.0),
        Point::new(4.0, 4.0),
        Point::new(3.0, 6.0),
        Point::new(0.0, 5.0),
    ];

    // act
    let hull = convex_hull(&points);

    // assert
    assert_eq!(hull, hull_points);
}

#[test]
fn return_input_if_count_below_3() {
    // arrange
    let points = vec![
        Point::new(3.0, 6.0),
        Point::new(3.0, 4.0),
        Point::new(0.0, 5.0),
    ];

    let hull_points = vec![
        Point::new(3.0, 6.0),
        Point::new(3.0, 4.0),
        Point::new(0.0, 5.0),
    ];

    // act
    let hull = convex_hull(&points);

    // assert
    assert_eq!(hull, hull_points);
}
