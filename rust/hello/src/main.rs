pub fn say_hi() -> String {

    String::from("Hello, World!")
}

fn main() {
    println!("{}", say_hi());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn said_hello_world() {
        assert_eq!(say_hi(), String::from("Hello, World!"));
    }

}
