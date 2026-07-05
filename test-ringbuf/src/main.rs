use ringbuf::{traits::*, StaticRb};

fn main() {
    let buf = [0u8; 20];
    let mut rb = StaticRb::<u8, 20>::from(buf);
    
    for i in 0..10 {
        rb.try_push(i).expect("Failed to push");
    }
    //rb.try_pop().unwrap();
    //rb.try_push(10).unwrap();
    //rb.try_push(11).unwrap();


    let (a, b) = rb.as_slices();
    for x in a.iter().chain(b.iter()) {
        println!("{}", x);
    }
}
