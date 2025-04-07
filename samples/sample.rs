#[derive(Debug)]
struct HasLifetime<'a> {
    some_string: &'a str,
}

fn do_something<'a>(has_lifetime: &HasLifetime<'a>) {
    println!("{}", has_lifetime.some_string);
}

fn main() {
    let has_lifetime = HasLifetime {
        some_string: "example",
    };
    do_something(&has_lifetime);
}

type TimeoutFunction = Box<Fn(&str, &ArgsMatcher) -> ()>;

fn whatever(&self, closure: Box<Fn() + 'static>) {
    // do nothing
}

pub unsafe fn hello() {}

fn main() {
    let a = 1;
    let mut b = 2;
    let c = &a;
    let d = &mut b;
    let mut e = &mut b;
}
