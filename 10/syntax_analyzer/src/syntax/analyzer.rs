use super::tokenizer::Token;

pub fn analyze_tokens(tokens: Vec<Token>) {
    for t in tokens.iter() {
        println!("{:?}", t);
    }
}