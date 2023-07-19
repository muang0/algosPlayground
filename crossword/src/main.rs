use orange_trees::{Tree, Node};

fn main() {
    println!("Hello, darling!");
}

fn generate_word_tree(wordlist: Vec<&str>) -> Option<Tree<i32, char>> {
    let mut guid = 0;
    let mut tree: Tree<i32, char> = Tree::new(Node::new(guid, ' '));
    let mut current_node = tree.root_mut();

    for word in wordlist {
        for char in word.chars() {
            guid += 1;
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
                let new_node = Node::new(guid, char);
                current_node.add_child(new_node);
                let nth = current_node.iter().len()-1;
                current_node = current_node.iter_mut().nth(nth).unwrap();
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
        let tree = generate_word_tree(wordlist);

        // compose & validate the generated word tree
        let binding = tree.unwrap();
        let first_node = &binding.root().children()[0];
        assert_eq!(*first_node.value(), 'f');
        let second_node: &Node<i32, char> = &first_node.children()[0];
        assert_eq!(*second_node.value(), 'r');
        let third_node: &Node<i32, char> = &second_node.children()[0];
        assert_eq!(*third_node.value(), 'e');
        let fourth_node_a: &Node<i32, char> = &third_node.children()[0];
        assert_eq!(*fourth_node_a.value(), 'd');
        let fourth_node_b: &Node<i32, char> = &third_node.children()[1];
        assert_eq!(*fourth_node_b.value(), 'y');
        let fourth_node_c: &Node<i32, char> = &third_node.children()[2];
        assert_eq!(*fourth_node_c.value(), 't');
        let fifth_node: &Node<i32, char> = &fourth_node_b.children()[0];
        assert_eq!(*fifth_node.value(), 'a');
    }
}
