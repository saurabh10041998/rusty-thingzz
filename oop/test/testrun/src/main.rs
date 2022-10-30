use testlib::AveragedCollection;

fn main() {
    let mut s = AveragedCollection::new();
    s.add(3);
    s.add(4);
    s.add(5);
    s.add(6);

    println!("[+] printing the collection: {:?}", s);

    s.remove();

    println!("[+] printing the collection: {:?}", s);
}
