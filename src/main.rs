fn main() {
    let mut ob = viper::StrTree::new();
    ob.build("abcde");
    ob.build("abcde");
    ob.build("pdd");
    let res = ob.query("abcde");
    println!("{} {}", res, ob.query("pdd"));
}
