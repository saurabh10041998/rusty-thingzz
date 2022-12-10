use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::mem::drop;

#[derive(Debug)]
pub struct Node {
    val: i32,
    points_to: Option<Weak<RefCell<Node>>>,
}



fn main()  { 
    let mut n1 = Node { val: 3, points_to : None };
    let n1_rc = Rc::new(RefCell::new(n1));
    // let mut n2 = Node { val: 4, points_to: Some(Rc::clone(&n1_rc))};      // Ok..
    let mut n2 = Node { val: 4, points_to: Some(Rc::downgrade(&n1_rc))};
    let n2_rc  = Rc::new(RefCell::new(n2));
    // n1_rc.borrow_mut().points_to = Some(Rc::clone(&n2_rc));               // Reference cycle no.!!!!
    n1_rc.borrow_mut().points_to = Some(Rc::downgrade(&n2_rc));

    println!("{:#?}", n1_rc);
    println!("{:#?}", n2_rc);

    // Strong counts
    println!("{}",Rc::strong_count(&n1_rc));        // 1 beacause of Rc::new
    println!("{}", Rc::strong_count(&n2_rc));       // 1 beacause of Rc::new

    // Weak counts
    println!("{}", Rc::weak_count(&n1_rc));         // 1 beacause of Rc::downgrade
    println!("{}", Rc::weak_count(&n2_rc));         // 1 beacause of Rc::downgrade

    // n1 -> n2(valid)
    if let Some(ref n1_produced_rc) = Rc::clone(&n1_rc).borrow().points_to.as_ref().unwrap().upgrade() {
        println!("The refernce is still valid {:?}", n1_produced_rc);
    }else {
        println!("[*] Gone!!!");
    }

    // delete n2_rc
    drop(n2_rc);


    // n1 -> n2 (invalid !!)
    if let Some(ref n1_produced_rc) = Rc::clone(&n1_rc).borrow().points_to.as_ref().unwrap().upgrade() {
        println!("The referece is still valid {:?}", n1_produced_rc);
    }else { 
        println!("[*] Gone!!");
    }

}