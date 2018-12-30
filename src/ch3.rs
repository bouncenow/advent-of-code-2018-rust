use crate::common::read_lines_from_file;

pub fn ch3() {
    let claims = read_claims("ch3.txt");
    println!("{}", count_overlapping_area(&claims));
    println!("{:?}", find_non_overlapping_claims(&claims));
}

const SIZE: usize = 1000;
const CLAIM_THRESHOLD: usize = 2;

fn count_overlapping_area(claims: &Vec<Claim>) -> usize {
    let mut buf: Vec<usize> = vec![0; SIZE * SIZE];

    for c in claims {
        c.mark(&mut buf);
    }

    let mut count = 0;
    for i in buf {
        if i >= CLAIM_THRESHOLD {
            count += 1;
        }
    }

    count
}

fn find_non_overlapping_claims<'a>(claims: &'a Vec<Claim>) -> Vec<&'a Claim> {
    let mut overlap = vec![false; claims.len()];
    for i in 0..(claims.len() - 1) {
        for j in (i + 1)..claims.len() {
            if claims[i].overlaps(&claims[j]) {
                overlap[i] = true;
                overlap[j] = true;
            }
        }
    }
    claims.iter().zip(overlap.iter())
        .filter(|(_, o)| !**o)
        .map(|(c, _)| c)
        .collect()
}

fn read_claims(file_name: &str) -> Vec<Claim> {
    read_lines_from_file(file_name)
        .iter()
        .map(|s| Claim::new(s))
        .collect()
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left_offset: usize,
    top_offset: usize,
    width: usize,
    height: usize
}

impl Claim {
    fn new(str: &str) -> Claim {
        let mut chars = str.chars();
        let id = parse_int(&mut chars).unwrap();
        let left_offset = parse_int(&mut chars).unwrap();
        let top_offset = parse_int(&mut chars).unwrap();
        let width = parse_int(&mut chars).unwrap();
        let height = parse_int(&mut chars).unwrap();

        Claim { id, left_offset, top_offset, width, height }
    }

    fn mark(&self, buf: &mut Vec<usize>) {
        for i in self.top_offset..(self.top_offset + self.height) {
            for j in self.left_offset..(self.left_offset + self.width) {
                let idx = i * SIZE + j;
                buf[idx] += 1;
            }
        }
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let other_lower = self.top_offset + self.height <= other.top_offset;
        let this_above = other.top_offset + other.height <= self.top_offset;
        let other_right = self.left_offset + self.width <= other.left_offset;
        let this_left = other.left_offset + other.width <= self.left_offset;
        !other_lower && !this_above && !other_right && !this_left
    }
}

fn parse_int(iter: &mut Iterator<Item=char>) -> Option<usize> {
    let mut result: Option<usize> = None;
    for c in iter.skip_while(|c| !c.is_digit(10)) {
        match c {
            '0'...'9' => {
                let num = c.to_digit(10).unwrap() as usize;
                result = match result {
                    Some(n) => Some(n * 10 + num),
                    None => Some(num)
                }
            }
            _ => return result
        }
    }
    return result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_overlap() {
        let c1 = Claim::new("#1 @ 1,3: 4x4");
        let c2 = Claim::new("#2 @ 3,1: 4x4");
        let c3 = Claim::new("#3 @ 5,5: 2x2");

        assert!(c1.overlaps(&c2));
        assert!(c2.overlaps(&c1));
        assert!(!c1.overlaps(&c3));
        assert!(!c3.overlaps(&c1));
        assert!(!c2.overlaps(&c3));
        assert!(!c3.overlaps(&c2));

        let c1 = Claim::new("#1 @ 0,0: 1x1");
        let c2 = Claim::new("#2 @ 1,1: 1x1");
        assert!(!c1.overlaps(&c2));
        assert!(!c2.overlaps(&c1));

        let c1 = Claim::new("#1 @ 0,1: 1x1");
        let c2 = Claim::new("#2 @ 3,1: 1x1");
        assert!(!c1.overlaps(&c2));
    }

}