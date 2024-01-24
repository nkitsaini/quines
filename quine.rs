// fn main() {
//     let mut a = r#"fn main() { let mut a = r#"{x}";
//         let mut b = a.replace("\"bhula\"", &("\"bhula\"".to_string() + &'#'.to_string()));
//         b = b.replace("bhula", a);
//         b = b.replace("b-----hula", "bhula");
//         b = b.replace(&"-".repeat(4), "bhula");
//         print!("{}", b);}"#;
//     let mut b = a.replace("\"{x}\"", &("\"{x}\"".to_string() + &'#'.to_string()));
//     b = b.replace("{x}", a);
//     b = b.replace("bhula", "{x}");
//     b = b.replace(&"-".repeat(4), "bhula");
//     print!("{}", b);
// }
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
