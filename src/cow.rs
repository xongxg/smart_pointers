use std::borrow::Cow;
use std::path::Prefix;

fn modulo_3(num: u8) -> Cow<'static, str> {
    match num % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(),
    }
}

#[test]
fn test_modulo_3() {
    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(message) => println!(
                "{} went in. The Cow is borrowed with this message: {}",
                number, message
            ),
            Cow::Owned(message) => println!(
                "{} went in. The Cow is owned with this message: {}",
                number, message
            ),
        }
    }
}

struct Item<'a, X: 'a>
where
    [X]: ToOwned<Owned = Vec<X>>,
{
    values: Cow<'a, [X]>,
}

impl<'a, X: Clone + 'a> Item<'a, X>
where
    [X]: ToOwned<Owned = Vec<X>>,
{
    fn new(values: Cow<'a, [X]>) -> Self {
        Item { values }
    }
}

#[test]
fn test_item() {
    let readonly = [1, 2];
    let borrowed = Item::new((&readonly[..]).into());
    match borrowed {
        Item {
            values: Cow::Borrowed(b),
        } => println!("borrowed {:?}", b),
        _ => panic!("expect borrowed value"),
    }

    let mut copy_on_write = borrowed;
    copy_on_write.values.to_mut().push(3);
    println!("copy_on_write = {:?}", copy_on_write.values);

    match copy_on_write {
        Item {
            values: Cow::Owned(_),
        } => println!("clone_on_write contains owned data"),
        _ => panic!("expect owned data"),
    }
}

fn add_prefix_by_cow<'a, T>(urls: T, prefix: &str) -> Vec<Cow<'a, String>>
where
    T: IntoIterator<Item = &'a String>,
{
    urls.into_iter()
        .map(|url| {
            if url.starts_with(prefix) {
                Cow::Borrowed(url)
            } else {
                Cow::Owned(format!("{}{}", prefix, url))
                // Cow::Owned(String::with_capacity(url.len() + prefix.len()) + prefix + url)
            }
        })
        .collect()
}

#[test]
fn test_add_prefix_by_cow() {
    let urls = vec!["127.0.0.1".to_string(), "https://127.0.0.9".to_string()];

    let res = add_prefix_by_cow(urls.as_slice(), "https://");
    println!("res = {:?}", res);
}
