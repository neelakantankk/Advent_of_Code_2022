use std::collections::HashMap;
use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let trees = parse_contents(&contents);
    let trees_visible = get_trees_visible(&trees);
    println!("The number of trees visible is {}", trees_visible);
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
            if *trees.get(&(i,tree.1)).unwrap()>= height {
                visible[0] = false;
                break;
            }
        }
        visible.push(true);
        for i in (tree.0+1..max_row+1).rev() {
            if *trees.get(&(i,tree.1)).unwrap() >= height {
                visible[1] = false;
                break;
            }
        }
        visible.push(true);
        for j in (0..tree.1).rev() {
            if *trees.get(&(tree.0,j)).unwrap() >= height {
                visible[2] = false;
                break;
            }
        }
        visible.push(true);
        for j in (tree.1+1..max_col+1).rev() {
            if *trees.get(&(tree.0,j)).unwrap() >= height {
                visible[3] = false;
                break;
            }
        }
        visible.iter().any(|x| *x)
    }
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
        assert_eq!(21,get_trees_visible(&trees));
    }

}
