use std::fmt::Debug;
pub trait Convex<K>: Ord + Copy + Debug
where
    K: PartialOrd,
{
    fn get_angle(a: &Self, b: &Self) -> K;
    fn get_turn(a: &Self, b: &Self, c: &Self) -> Turn;
}

#[derive(PartialEq, PartialOrd)]
pub enum Turn {
    Clockwise,
    CounterClockwise,
}

// Convex hull
// Take array points. Output array of points that make the convex hull.

// Find the lowest left most point
// Calculate angle between it and every point
// Sort the list from smallest to highest and turn into sorted_stack

// Create vector called stack add first two vals from sorted values
// Foreach point in sorted value check that first two top most vals and point
// form counter clockwise (ccw) angle

// If clockwise remove top value from stack and repeat.
// If ccw add point to stack and move on to next point.

// Once all points checked return stack vector

pub fn convex_hull<T, K>(source: &[T]) -> Vec<T>
where
    T: Convex<K>,
    K: PartialOrd,
{
    if source.len() <= 3 {
        return source.to_vec();
    }

    let min = source.iter().min().unwrap();

    let sorted = sort_by_angle(source, min);

    let mut stack = vec![sorted[0], sorted[1]];

    for p in sorted.iter().skip(2) {
        while stack.len() > 1
            && Turn::Clockwise
                == Convex::get_turn(&stack[stack.len() - 2], &stack[stack.len() - 1], p)
        {
            stack.pop();
        }
        stack.push(*p);
    }

    stack
}

fn sort_by_angle<T, K>(source: &[T], corner: &T) -> Vec<T>
where
    T: Convex<K>,
    K: PartialOrd,
{
    let mut angles: Vec<MinScored<K, T>> = source
        .iter()
        .map(|p| {
            let angle = Convex::get_angle(corner, p);

            return MinScored {
                key: angle,
                value: *p,
            };
        })
        .collect();

    //Sort values by angle to corner.
    angles.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    return angles.iter().map(|ms| ms.value).collect();
}

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>
where
    T: Convex<K>,
    K: PartialOrd,
{
    pub key: K,
    pub value: T,
}
