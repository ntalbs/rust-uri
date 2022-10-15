mod uri;

use crate::uri::Uri;

fn main() {
    let uri = Uri::from_str("http://ntalbs.github.io:80/hello/world?a=10&b=20#frag").unwrap();
    println!("{}", &uri);
    println!("{:?}", &uri);
}
