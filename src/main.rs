fn tokenize(expression: &str) -> Vec<String> {
    return expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split(" ")
        .map(|s| s.to_string())
        .collect();
}

fn main() {
    let program: &str = "(begin (define r 10) (* r r)";
    println!("{}", program);
    let tokenized: Vec<String> = tokenize(program);

    for t in tokenized {
        println!("{}", t)
    }
}
