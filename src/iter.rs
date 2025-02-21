use std::fmt::Debug;
use std::mem::transmute;

struct EvenNumbers {
    count: usize,
    limit: usize,
}

impl Iterator for EvenNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit {
            return None;
        }

        let res = self.count * 2;
        self.count += 1;
        Some(res)
    }
}

#[test]
fn test_even_numbers() {
    let nums = EvenNumbers {
        count: 1,
        limit: 10,
    };
    for (_, v) in nums.enumerate() {
        println!("v: {}", v);
    }

    // let res = nums.take(10).collect::<Vec<_>>();
    // println!("res: {:?}", res);

    let v = vec![1, 2, 3, 4];
    let a: &Vec<u64> = &v;
    // 转为 trait object
    let b: &dyn Debug = &v;
    println!("a: {}", a as *const _ as usize);
    println!("b: {:?}", unsafe { transmute::<_, (usize, usize)>(b) });
}
