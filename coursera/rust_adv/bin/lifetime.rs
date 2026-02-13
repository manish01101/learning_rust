/*

tracking how long references are valid so you don’t get dangling pointers.
lifetime denoted by apostrophe


What 'a means:
Both x and y live at least as long as 'a
The returned reference also lives at most as long as 'a


Lifetimes do not exist at runtime
They are only for the compiler
They do not affect performance
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
