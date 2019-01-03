use crate::common::read_file;

pub fn ch8() {
    let tree_nums: Vec<_> = read_file("ch8.txt")
        .trim()
        .to_string()
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let tree = TreeNode::parse(&tree_nums).unwrap();
    println!("Meta data entries sum: {}", tree.meta_data_entries_sum());
    println!("Node sum value for root node: {}", tree.node_value());
}

struct TreeNode {
    children: Vec<TreeNode>,
    meta_data: Vec<i32>,
}

impl TreeNode {
    fn parse(tree_nums: &[i32]) -> Result<TreeNode, String> {
        let (tree_node, _) = TreeNode::parse_tree_from_beginning(tree_nums)?;
        Ok(tree_node)
    }

    fn parse_tree_from_beginning(tree_nums: &[i32]) -> Result<(TreeNode, &[i32]), String> {
        if tree_nums.len() >= 2 {
            let child_count = tree_nums[0] as usize;
            let meta_data_count = tree_nums[1] as usize;
            let mut to_parse = &tree_nums[2..];
            let mut children = Vec::with_capacity(child_count);
            for _ in 0..child_count {
                let (child, left) = TreeNode::parse_tree_from_beginning(to_parse)?;
                children.push(child);
                to_parse = left;
            }
            if to_parse.len() < meta_data_count {
                return Err(format!(
                    "Expected at least {} meta data elements, found only {}",
                    meta_data_count,
                    to_parse.len()
                ));
            }
            return Ok((
                TreeNode {
                    children,
                    meta_data: to_parse[0..meta_data_count].to_vec(),
                },
                &to_parse[meta_data_count..],
            ));
        } else {
            return Err("Expected header for child count and meta data count".to_string());
        }
    }

    fn meta_data_entries_sum(&self) -> i32 {
        let children_sum = self
            .children
            .iter()
            .map(|n| n.meta_data_entries_sum())
            .sum::<i32>();
        let this_sum = self.meta_data.iter().sum::<i32>();
        children_sum + this_sum
    }

    fn node_value(&self) -> i32 {
        if self.children.is_empty() {
            return self.meta_data.iter().sum::<i32>();
        }
        self.meta_data
            .iter()
            .map(|m| match m {
                0 => 0,
                i => match self.children.get((i - 1) as usize) {
                    Some(c) => c.node_value(),
                    None => 0,
                },
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_tree() -> TreeNode {
        let tree_nums = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        TreeNode::parse(&tree_nums).unwrap()
    }

    #[test]
    fn test_meta_data_entries_sum() {
        let tree = get_test_tree();
        assert_eq!(138, tree.meta_data_entries_sum());
    }

    #[test]
    fn test_node_value() {
        let tree = get_test_tree();
        assert_eq!(66, tree.node_value());
    }
}
