//Crates: Modules that produce a library
//or executable
//Modules : Organise and handle privacy
//Packages: Build, test and share crates
//Paths: A way of naming an item such as a struct,
//function
mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }
    pub mod help_customer {
        use std::io;

        fn seat_at_table() {
            println!("You may be seated at a table now");
        }
        pub fn take_order() {
            seat_at_table();
            println!("What kind of toppings do you want?");
            let mut toppings = String::new();
            io::stdin()
                .read_line(&mut toppings)
                .expect("Did not get a topping in a string format");
            println!("{}", toppings);
            let customer_pizza: super::Pizza = super::Pizza::lunch(&toppings.trim_end());
            serve_customer(customer_pizza)
        }
        fn serve_customer(customer_pizza: super::Pizza) {
            println!(
                "The customer is served a regular pizza with {}",
                customer_pizza.topping
            );
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order()
}
