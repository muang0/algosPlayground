use orange_trees::{Tree, Node};

fn main() {
    println!("Hello, darling!");
}

fn generate_word_tree(wordlist: Vec<&str>) -> Option<Tree<&str, char>> { // , Vec<Node<&str, char>>
    let mut tree: Tree<&str, char> = Tree::new(Node::new("root", ' '));
    let mut current_node = tree.root_mut();
    // let mut nodes: Vec<Node<&str, char>> = vec![];

    for word in wordlist {
        for char in word.chars() {
            let mut char_exists = false;
            let mut matched_child_node_index = 0;
            for (index, node) in current_node.iter().enumerate() {
                if char == *node.value() {
                    char_exists = true;
                    matched_child_node_index = index;
                    break
                }
            }
            if char_exists {
                current_node = current_node.iter_mut().nth(matched_child_node_index).unwrap();
            } else {
                let new_node = Node::new("asdf", char);
                current_node.add_child(new_node);
                // nodes.append(&mut vec![new_node]);
                let nth = current_node.iter().len()-1;
                current_node = current_node.iter_mut().nth(nth).unwrap(); // TODO
            }
        }
        current_node = tree.root_mut();
    }
    return Some(tree)
}

#[cfg(test)]
mod tests {
    use crate::{Node, generate_word_tree};

    #[test]
    fn map_words_to_tree() {
        // wordlist
        let wordlist = vec!["fred", "freya", "fret"];

        // wordtree

        // compose & validate the generated word tree
        let test = generate_word_tree(wordlist.clone());
        let test = generate_word_tree(wordlist);
    }
}
