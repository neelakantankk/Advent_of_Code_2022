use std::collections::HashMap;
use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let trees = parse_contents(&contents);
    let trees_visible = get_trees_visible(&trees);
    println!("The number of trees visible is {}", trees_visible);
    let (highest_scenic_score, _) = get_highest_scenic_score(&trees);
    println!(
        "The highest possible scenic score is {}",
        highest_scenic_score
    );
    Ok(())
}

fn parse_contents(contents: &str) -> HashMap<(usize, usize), u32> {
    let mut trees: HashMap<(usize, usize), u32> = HashMap::new();
    for (row, line) in contents.lines().enumerate() {
        for (col, tree) in line.chars().enumerate() {
            trees.insert((row, col), tree.to_digit(10).unwrap());
        }
    }
    trees
}

fn get_trees_visible(trees: &HashMap<(usize, usize), u32>) -> usize {
    let mut trees_visible = 0;
    let max_row = trees.keys().map(|x| x.0).max().unwrap();
    let max_col = trees.keys().map(|x| x.1).max().unwrap();
    for (tree, height) in trees {
        if is_visible(trees, *tree, *height, max_row, max_col) {
            trees_visible += 1;
        }
    }
    trees_visible
}

fn is_visible(
    trees: &HashMap<(usize, usize), u32>,
    tree: (usize, usize),
    height: u32,
    max_row: usize,
    max_col: usize,
) -> bool {
    if tree.0 == 0 || tree.0 == max_row || tree.1 == 0 || tree.1 == max_col {
        true
    } else {
        let mut visible = Vec::new();
        visible.push(true);
        for i in (0..tree.0).rev() {
            if *trees.get(&(i, tree.1)).unwrap() >= height {
                visible[0] = false;
                break;
            }
        }
        visible.push(true);
        for i in (tree.0 + 1..max_row + 1).rev() {
            if *trees.get(&(i, tree.1)).unwrap() >= height {
                visible[1] = false;
                break;
            }
        }
        visible.push(true);
        for j in (0..tree.1).rev() {
            if *trees.get(&(tree.0, j)).unwrap() >= height {
                visible[2] = false;
                break;
            }
        }
        visible.push(true);
        for j in (tree.1 + 1..max_col + 1).rev() {
            if *trees.get(&(tree.0, j)).unwrap() >= height {
                visible[3] = false;
                break;
            }
        }
        visible.iter().any(|x| *x)
    }
}

fn get_highest_scenic_score(trees: &HashMap<(usize, usize), u32>) -> (usize, (usize, usize)) {
    let mut highest_scenic_score = 0;
    let mut max_tree = (0, 0);
    let max_row = trees.keys().map(|x| x.0).max().unwrap();
    let max_col = trees.keys().map(|x| x.1).max().unwrap();
    for tree in trees.keys() {
        let score = get_scenic_score(trees, *tree, max_row, max_col);
        if score > highest_scenic_score {
            highest_scenic_score = score;
            max_tree = *tree;
        }
    }
    (highest_scenic_score, max_tree)
}

fn get_scenic_score(
    trees: &HashMap<(usize, usize), u32>,
    tree: (usize, usize),
    max_row: usize,
    max_col: usize,
) -> usize {
    let mut score = 1;
    let res = match (0..tree.0)
        .rev()
        .map(|i| (i, tree.1))
        .position(|ct| trees.get(&ct).unwrap() >= trees.get(&tree).unwrap())
    {
        None => tree.0,
        Some(i) => i + 1,
    };
    score *= res;
    let res = match (0..tree.1)
        .rev()
        .map(|i| (tree.0, i))
        .position(|ct| trees.get(&ct).unwrap() >= trees.get(&tree).unwrap())
    {
        None => tree.1,
        Some(i) => i + 1,
    };
    score *= res;
    let res = match (tree.0 + 1..max_row + 1)
        .map(|i| (i, tree.1))
        .position(|ct| trees.get(&ct).unwrap() >= trees.get(&tree).unwrap())
    {
        None => max_row - tree.0,
        Some(i) => i + 1,
    };
    score *= res;
    let res = match (tree.1 + 1..max_col + 1)
        .map(|i| (tree.0, i))
        .position(|ct| trees.get(&ct).unwrap() >= trees.get(&tree).unwrap())
    {
        None => max_col - tree.1,
        Some(i) => i + 1,
    };
    score *= res;
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_data() -> &'static str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn test_is_visible() {
        let data = make_data();
        let trees = parse_contents(data);
        assert_eq!(21, get_trees_visible(&trees));
    }
    #[test]
    fn test_get_scenic_score() {
        let data = make_data();
        let trees = parse_contents(data);
        let max_row = trees.keys().map(|x| x.0).max().unwrap();
        let max_col = trees.keys().map(|x| x.1).max().unwrap();
        println!("Tree: (3,2)");
        let score = get_scenic_score(&trees, (3, 2), max_row, max_col);
        assert_eq!(8, score);
        println!("Tree: (1,2)");
        let score2 = get_scenic_score(&trees, (1, 2), max_row, max_col);
        assert_eq!(4, score2);
    }
    #[test]
    fn test_highest_score() {
        let data = make_data();
        let trees = parse_contents(data);
        let (score, tree) = get_highest_scenic_score(&trees);
        println!("Got {:?}", tree);
        assert_eq!(8, score);
    }
}
