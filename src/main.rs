use make_lang::interpreter::eval_statement;
use make_lang::parser::{make_syntax_tree, read_tokens_sequence_of_source, traverse_syntax_tree};
use make_lang::primitives::Token;
use make_lang::primitives::{ASTNode, TokenType};
use make_lang::traits::OptionStringify;
use make_lang::{lexer::tokenize, parser::pair_tokens};

use std::{fs::File, io::Read, io::Write};

fn main() -> std::io::Result<()> {
    // Tokenize and write to file
    {
        let mut language_file: File = File::open("language.test")?;
        let language_file_contents: String = {
            let mut contents: String = String::new();
            language_file.read_to_string(&mut contents)?;

            contents
        };

        let tokens: Vec<Token> = tokenize(language_file_contents);

        let tokens_as_vec_string: Vec<String> = tokens
            .iter()
            .map(|token| {
                // Removes `Some(..)` to become `..`
                if token.kind == TokenType::STRING {
                    token.value.option_as_string()
                } else {
                    token.as_string()
                }
            })
            .collect();

        let tokens_as_string = tokens_as_vec_string.join("\n");

        let mut tokens_file: File = File::create("tokens.txt")?;
        let _ = tokens_file.write_all(tokens_as_string.as_bytes());
    }

    let tokens: Vec<Token> = pair_tokens(read_tokens_sequence_of_source()?);

    let root_node_of_syntax_tree: ASTNode = make_syntax_tree(tokens)?;
    // traverse_syntax_tree(root_node_of_syntax_tree);

    // EVAL LOOP
    let mut current_node: ASTNode = root_node_of_syntax_tree;
    loop {
        eval_statement(current_node.left_child.clone());
        if current_node.get_next_node().is_none() {
            break;
        }

        current_node = current_node.get_next_node().unwrap();
    }

    Ok(())
}
