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















    // 20.1 Visibility
    // Modules are a privacy boundary:

    // Module items are private by default (hides implementation details).
    // Parent and sibling items are always visible.
    // In other words, if an item is visible in module foo, it’s visible in all the descendants of foo.

    mod outer {
        fn private() {
            println!("outer::private");
        }

        pub fn public() {
            println!("outer::public");
        }

        mod inner {
            fn private() {
                println!("outer::inner::private");
            }

            pub fn public() {
                println!("outer::inner::public");
                super::private();
            }
        }
    }

    outer::public();
}
