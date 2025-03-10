mod book;
mod cells;
mod person;
mod refcell;
mod list;
mod ref_t;
mod node;
mod drop;
mod deref;
mod rc;
mod lifetime;
mod cow;
mod intoiter;
mod iter;
mod inheritance;

fn main() {
    let x = 42;
    let ptr = &x as *const _;
    unsafe {
        println!("{}", *ptr);
    }

    drop(x);
    let y = unsafe { *ptr };
    println!("{}", y);
}
