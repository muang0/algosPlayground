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

#[derive(Clone, Debug, PartialEq)]
struct Board {
    nodes: Vec<Vec<Space>>
}

#[derive(Clone, Debug, PartialEq)]
struct Space {
    value: char,
    in_word: bool
}

impl Board {
    fn new(raw_board: Vec<Vec<char>>) -> Self {
        let nodes: Vec<Vec<Space>> = raw_board.iter().map(|column: &Vec<char>| column.iter().map(|val| Space{value: *val, in_word: false}).collect()).collect();
        return Board{nodes};    
    }
}

fn play_game(board: &Board, tree: &Tree<i32, char>) -> Board {
    let directions = [[0,1], [-1,1], [-1,0], [-1,-1], [0,-1], [1,-1], [1,0], [1,1]];
    let mut new_board = board.clone();
    // traverse the characters on the board
    for (y_index, x_arr) in board.nodes.iter().enumerate() {
        for (x_index, space) in x_arr.iter().enumerate() {
            // check if the current character is the leading char in any words
            for node in tree.root().children() {
                if *node.value() == space.value {
                    let current_node = node;
                    // for each of the 8 possible directions for words
                    for direction in directions {
                        let mut new_y_index = y_index as i32;
                        let mut new_x_index = x_index as i32;
                        let mut tmp_current_node = current_node;
                        let mut y_x_footsteps: Vec<[i32; 2]> = vec![[y_index as i32, x_index as i32]];
                        // follow the given direction until off the board, word is found, or a char doesn't match any words
                        'a: loop {
                            new_y_index += direction[0];
                            new_x_index += direction[1];
                            let mut no_children_matched = true;
                            // determine if the next char on the board matches the next char from on the target word
                            'b: for child_node in tmp_current_node.children().iter() {
                                if new_x_index >= 0 && new_y_index >= 0 {
                                    if let Some(x_arr_two) = board.nodes.iter().nth(new_y_index.try_into().unwrap()) {
                                        if let Some(target_space) = x_arr_two.iter().nth(new_x_index.try_into().unwrap()) {
                                            if *child_node.value() == target_space.value {
                                                y_x_footsteps.append(&mut vec![[new_y_index, new_x_index]]);
                                                no_children_matched = false;
                                                tmp_current_node = child_node;
                                                // if the current board char is the end of a wordlist word, mark the word's chars accordingly on the board struct
                                                if tmp_current_node.is_leaf() {
                                                    for footstep in y_x_footsteps {
                                                        new_board.nodes.iter_mut().nth(footstep[0].try_into().unwrap()).unwrap().iter_mut().nth(footstep[1].try_into().unwrap()).unwrap().in_word = true;
                                                    }
                                                    break 'a
                                                }
                                                break 'b
                                            }        
                                        }
                                    }    
                                }
                            } if no_children_matched {
                                break 'a
                            }
                        }
                    }
                }
            }
        }
    }
    return new_board;
}

#[cfg(test)]
mod tests {
    use crate::{Node, generate_word_tree, play_game};

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

    #[test]
    fn finds_word() {
        // compose search tree
        let wordlist = vec!["fred", "freya", "fret"];
        let mut tree = generate_word_tree(wordlist).unwrap();

        // compose board
        let mut board = crate::Board::new(vec![
            vec!['f', 'r', 'e', 'd'],
            vec!['f', 'r', 'f', 'f'],
            vec!['f', 'f', 'e', 'f'],
            vec!['f', 'f', 'f', 't']
        ]);

        // play_game()
        let res_board: crate::Board = play_game(&mut board, &mut tree);
        
        // validate game results
        let expected_word_spaces = [[0,0], [0,1], [0,2], [0,3], [1,1], [2,2], [3,3]];
        for [y_index, x_index] in expected_word_spaces {
            board.nodes.iter_mut().nth(y_index).unwrap().iter_mut().nth(x_index).unwrap().in_word = true;
        }
        assert_eq!(res_board, board);
    }
}
