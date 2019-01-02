use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::read_lines_from_file;

const TOTAL_DISTANCE_LESS_THAN: i32 = 10000;

pub fn ch6() {
    let points: Vec<_> = read_lines_from_file("ch6.txt")
        .iter()
        .map(|l| Point::parse(l).unwrap_or_else(|e| panic!("Error during parsing point: {}", e)))
        .collect();

    println!("max finite area: {}", find_max_finite_area(&points));
    println!(
        "area of total distance to all points less than {}: {}",
        TOTAL_DISTANCE_LESS_THAN,
        find_area_of_total_distance_less_than(&points, TOTAL_DISTANCE_LESS_THAN)
    );
}

fn find_area_of_total_distance_less_than(points: &[Point], distance: i32) -> usize {
    if points.is_empty() {
        return 0;
    }

    let points_stats = PointsStats::new(points);

    let mut area = 0;
    for x in (points_stats.min_x - distance)..=(points_stats.max_x + distance) {
        for y in (points_stats.min_y - distance)..=(points_stats.max_y + distance) {
            if points_stats.total_distance(&Point::new(x, y)) < distance {
                area += 1;
            }
        }
    }

    area
}

fn find_max_finite_area(points: &[Point]) -> usize {
    if points.is_empty() {
        return 0;
    }
    let points_stats = PointsStats::new(points);
    let points_with_finite_area = points_stats.find_points_with_finite_area();
    let mut points_by_area: HashMap<_, usize> = HashMap::new();
    for y in points_stats.min_y..(points_stats.max_y + 1) {
        for x in points_stats.min_x..(points_stats.max_x + 1) {
            let this = Point::new(x, y);
            if let Some(p) = this.closest(points) {
                if points_with_finite_area.contains(p) {
                    let p_entry = points_by_area.entry(p).or_insert(0);
                    *p_entry += 1;
                }
            }
        }
    }
    points_by_area
        .iter()
        .max_by_key(|&(_, a)| a)
        .map(|(_, a)| *a)
        .expect("At least one point with area")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn parse(str: &str) -> Result<Point, &'static str> {
        match scan_fmt!(str, "{}, {}", i32, i32) {
            (Some(x), Some(y)) => Ok(Point::new(x, y)),
            _ => Err("Couldn't parse point"),
        }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn closest<'p>(&self, points: &'p [Point]) -> Option<&'p Point> {
        let mut closest = None;
        let mut tied = false;
        for p in points {
            let this_distance = self.manhattan_distance(p);
            if let Some((_, distance)) = closest {
                if this_distance < distance {
                    closest = Some((p, this_distance));
                    tied = false;
                } else if this_distance == distance {
                    tied = true;
                }
            } else {
                closest = Some((p, this_distance));
            }
        }
        match (closest, tied) {
            (Some((closest, _)), false) => Some(closest),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PointsStats<'a> {
    points: &'a [Point],
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl<'a> PointsStats<'a> {
    fn new(points: &'a [Point]) -> PointsStats<'a> {
        if points.is_empty() {
            panic!("There should be at least one point");
        }
        let min_x = points.iter().min_by_key(|p| p.x).map(|p| p.x).unwrap();
        let min_y = points.iter().min_by_key(|p| p.y).map(|p| p.y).unwrap();
        let max_x = points.iter().max_by_key(|p| p.x).map(|p| p.x).unwrap();
        let max_y = points.iter().max_by_key(|p| p.y).map(|p| p.y).unwrap();
        PointsStats {
            points,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn find_points_with_finite_area(&self) -> HashSet<Point> {
        let mut points_closest_to_border = HashSet::new();
        for x in self.min_x..=self.max_x {
            let p1 = Point::new(x, self.min_y);
            let p2 = Point::new(x, self.max_y);
            self.add_closest_point_to_set_if_any(&p1, &mut points_closest_to_border);
            self.add_closest_point_to_set_if_any(&p2, &mut points_closest_to_border);
        }
        for y in self.min_y..=self.max_y {
            let p1 = Point::new(self.min_x, y);
            let p2 = Point::new(self.max_x, y);
            self.add_closest_point_to_set_if_any(&p1, &mut points_closest_to_border);
            self.add_closest_point_to_set_if_any(&p2, &mut points_closest_to_border);
        }
        self.points
            .iter()
            .filter(|p| !points_closest_to_border.contains(p))
            .map(|p| p.clone())
            .collect()
    }

    fn add_closest_point_to_set_if_any(&self, p: &Point, set: &mut HashSet<Point>) {
        if let Some(closest) = p.closest(self.points) {
            set.insert(closest.clone());
        }
    }

    fn total_distance(&self, p: &Point) -> i32 {
        self.points.iter().map(|sp| p.manhattan_distance(sp)).sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_finite_area() {
        let points = [
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];

        assert_eq!(17, find_max_finite_area(&points));

        //x---x
        //--x--
        //x---x
        let points = [
            Point::new(1, 1),
            Point::new(1, 5),
            Point::new(3, 3),
            Point::new(5, 1),
            Point::new(5, 5),
        ];
        assert_eq!(5, find_max_finite_area(&points));
    }

}
