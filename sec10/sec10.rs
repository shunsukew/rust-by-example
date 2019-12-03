use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

mod my {
    // 関数
    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn function() {
        println!("called `my::function()`");
    }

    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }


    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }

        mod private_nested {
            #[allow(dead_code)]
            pub fn function() {
                println!("called `my::private_nested::function()`");
            }
        }
    }

    // 構造体
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }

    mod cool {
        pub fn function() {
           println!("called `cool::function()`"); 
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");
        self::function();
        function();

        self::cool::function();

        super::function();
    }
}

fn main() {
    function();
    my::function();

    my::indirect_access();
    my::nested::function();

    let whitebox = my::WhiteBox { contents: "public information" };
    println!("The white box contains: {}", whitebox.contents);

    let _black_box = my::BlackBox::new("classified information");

    other_function();
    println!("Entering block");
    {
        use deeply::nested::function;
        function();

        println!("Leaving block");
    }

    function();

    my::indirect_call();
}
