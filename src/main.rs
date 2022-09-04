mod uri;

use crate::uri::Scanner;

fn main() {
    for token in Scanner::new("https://ntalbs.github.io:8443/hello/world/").tokens() {
        println!("{:?}", token);
    };
}
