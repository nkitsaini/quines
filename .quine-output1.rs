fn main() {
        let mut a = r#"fn main() {
        let mut a = r#"{a}"{b};
        let a_repl = '{'.to_string() + &'a'.to_string() + &'}'.to_string();
        let b_repl = '{'.to_string() + &'b'.to_string() + &'}'.to_string();
        let b = a.replace(&b_repl, &'#'.to_string());
        let c = b.replace(&a_repl, a);
        println!("{}", c);
    }"#;
        let a_repl = '{'.to_string() + &'a'.to_string() + &'}'.to_string();
        let b_repl = '{'.to_string() + &'b'.to_string() + &'}'.to_string();
        let b = a.replace(&b_repl, &'#'.to_string());
        let c = b.replace(&a_repl, a);
        println!("{}", c);
    }
