#![feature(phase)]

#[phase(plugin)]
extern crate excessive_bools_lint;

struct Foo {
    is_leaving_session: bool,
    is_connecting: bool,
    is_doing_stuff: bool,
}

fn main() {
    let f = Foo {
        is_leaving_session: true,
        is_connecting: true,
        is_doing_stuff: false,
    };

    if f.is_leaving_session || f.is_connecting {
        println!("You are changing a state");
    } else if f.is_doing_stuff {
        println!("The thing is busy");
    }
}
