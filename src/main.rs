mod book;
mod cells;
mod person;
mod refcell;
mod list;
mod ref_t;

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
