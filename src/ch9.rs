use crate::common::read_file;

pub fn ch9() {
    let (players, marbles) = read_input(&read_file("ch9.txt")).unwrap();
    println!("{} players, {} marbles", players, marbles);
    println!(
        "winning score for {}, {}: {}",
        players,
        marbles,
        winning_score(players, marbles)
    );
    println!(
        "winning score for {}, {}: {}",
        players,
        marbles * 100,
        winning_score(players, marbles * 100)
    );
}

fn read_input(str: &str) -> Option<(usize, usize)> {
    match scan_fmt!(
        str,
        "{} players; last marble is worth {} points",
        usize,
        usize
    ) {
        (Some(players), Some(marbles)) => Some((players, marbles)),
        _ => None,
    }
}

const CHECKPOINT: usize = 23;
const REMOVE_OFFSET: usize = 7;

#[derive(Clone)]
struct Node {
    previous: usize,
    next: usize,
}

impl Node {
    fn default() -> Node {
        Node {
            next: 0,
            previous: 0,
        }
    }

    fn new(previous: usize, next: usize) -> Node {
        Node { next, previous }
    }
}

fn winning_score(players: usize, marbles: usize) -> usize {
    let mut marbles_list = vec![Node::default(); marbles + 1];
    let mut players_scores = vec![0; players];

    marbles_list[0] = Node::new(1, 1);
    marbles_list[1] = Node::new(0, 0);

    let mut current_node = 1;

    for to_insert in 2..=marbles {
        match to_insert % CHECKPOINT {
            0 => {
                let mut to_remove_ptr = current_node;
                for _ in 0..REMOVE_OFFSET {
                    to_remove_ptr = marbles_list[to_remove_ptr].previous;
                }
                let to_remove_node = marbles_list[to_remove_ptr].clone();
                let to_remove_previous_node = marbles_list[to_remove_node.previous].clone();

                marbles_list[to_remove_node.previous] =
                    Node::new(to_remove_previous_node.previous, to_remove_node.next);

                players_scores[to_insert % players] += to_remove_ptr + to_insert;
                current_node = to_remove_node.next;
            }
            _ => {
                let next_idx = marbles_list[current_node].next;
                let next_next = marbles_list[next_idx].next;

                marbles_list[to_insert] = Node::new(next_idx, next_next);
                marbles_list[next_idx] = Node::new(current_node, to_insert);

                current_node = to_insert;
            }
        }
    }

    players_scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_score() {
        assert_eq!(32, winning_score(9, 25));
        assert_eq!(8317, winning_score(10, 1618));
        assert_eq!(146373, winning_score(13, 7999));
        assert_eq!(2764, winning_score(17, 1104));
    }
}
