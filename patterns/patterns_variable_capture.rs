enum Message {
    Hello {id: i32}
}

fn main() {
    let msg = Message::Hello{id: 5};
    
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found the id variable in range: {}", id_variable);
        },
        Message::Hello{id : 10..=12} => {
            println!("Found id in some other range");
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
    
}
