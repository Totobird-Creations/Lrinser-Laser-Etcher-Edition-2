use super::node::Node;

pub fn load<S : Into<String>>(url_part : S) -> Node {
    let url = format!("https://www.desmos.com/calculator{}", url_part.into());
}
