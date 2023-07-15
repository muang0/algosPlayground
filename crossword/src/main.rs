// TODO: clean up unwrap()

fn main() {
    println!("Hello, world!");
}

struct Node<'a> {
    value: Option<char>,
    nodes: Option<Vec<&'a Node<'a>>>
}

fn generate_word_tree(wordlist: Vec<&str>) -> Option<Node<'static>> {
    let mut global_root: Node = Node{value: None, nodes: Some(vec![])};
    let mut current_node = &mut global_root;
    for word in wordlist {
        for char in word.chars() {
            'a: for mut node in current_node.nodes.as_mut().unwrap() {
                if char == node.value.unwrap() {
                    current_node = node;
                    break 'a
                }
            }
            // Do we need to block this with a condition check set in lines 19-20
            // insert node with char, make that one current_node
            let mut new_node = Node{value: Some(char), nodes: Some(vec![])};
            current_node.nodes.unwrap().append(&mut vec![&new_node]);
            // current node = newly-created child node
            let current_node = &mut new_node;
        }
    }
    return Some(global_root)
}

#[cfg(test)]
mod tests {
    use crate::{Node, generate_word_tree};

    #[test]
    fn map_words_to_tree() {
        // wordlist
        let wordlist = vec!["fred", "freya", "fret"];

        // wordtree
        let n3_3 = Node{value: Some('t'), nodes: None};
        let n2_4: Node<'_> = Node{value: Some('a'), nodes: None};
        let n2_3: Node<'_> = Node{value: Some('y'), nodes: Some(vec![&n2_4])};
        let n1_3: Node<'_> = Node{value: Some('d'), nodes: None};
        let n0_2: Node<'_> = Node{value: Some('e'), nodes: Some(vec![&n1_3, &n2_3, &n3_3])};
        let n0_1: Node<'_> = Node{value: Some('r'), nodes: Some(vec![&n0_2]) };
        let n0_0: Node<'_> = Node{value: Some('f'), nodes: Some(vec![&n0_1]) };
        let wordtree = Node{value: None, nodes: Some(vec![&n0_0]) };

        // compose & validate the generated word tree
        let Some(word_tree) = generate_word_tree(wordlist);

    }
}
