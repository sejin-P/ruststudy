fn main() {
    // 20 Modules
    mod foo {
        pub fn do_something() {
            println!("in the foo module");
        }
    }

    mod bar {
        pub fn do_something() {
            println!("in the bar module");
        }
    }

    foo::do_something();
    bar::do_something();
}
