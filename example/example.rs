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

    lots_of_args("hello world", true, false, 1, true, false);
}

fn lots_of_args(foo: &str, bar: bool, baz: bool, not_bool: uint, maybe_bool: bool, quux: bool) {
    println!("Arguments are {}, {}, {}, {}, {}, {}", foo, bar, baz, not_bool, maybe_bool, quux)
}
