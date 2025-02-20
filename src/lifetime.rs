struct Context<'a>(&'a str);

struct Parser<'a, 'b: 'a> {
    context: &'a Context<'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&self) -> Result<(), &'b str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

#[test]
fn test_subtyping() {
    let context = Context("foobar");
    let parser = Parser { context: &context };
    let res = parser.parse();
    println!("{:?}", res);
}

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

#[test]
fn test_subtyping2() {
    let ref_1 = Ref(&"foobar");
    println!("{:?}", ref_1);
}

trait Red {}

#[derive(Debug)]
struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

#[test]
fn test_subtyping3() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
    // println!("{:?}", obj);
}
