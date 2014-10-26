# Rust Excessive Bools Int

Some not so far time ago there was a programmer who was presented with an
"emergent state machine" of half a gazillion bools.  "Never again should
someone undergo such suffering", he proclaimed and wrote a rustc plugin
so that future generations would be saved from such atrocities.

What it looks like:

```
example.rs:6:1: 10:2 warning: Struct contains an excessive number (3) of bools
  (is_leaving_session, is_connecting, is_doing_stuff).  Did you want to create
  a state machine?, #[warn(excessive_bool_usage)] on by default
example.rs:6 struct Foo {
example.rs:7     is_leaving_session: bool,
example.rs:8     is_connecting: bool,
example.rs:9     is_doing_stuff: bool,
example.rs:10 }
example.rs:7:5: 7:29 note: boolean field defined here
example.rs:7     is_leaving_session: bool,
                 ^~~~~~~~~~~~~~~~~~~~~~~~
example.rs:8:5: 8:24 note: boolean field defined here
example.rs:8     is_connecting: bool,
                 ^~~~~~~~~~~~~~~~~~~
example.rs:9:5: 9:25 note: boolean field defined here
example.rs:9     is_doing_stuff: bool,
                 ^~~~~~~~~~~~~~~~~~~~
```
