pub mod hosting;
pub mod service_order;



#[derive(Debug)]
pub struct Breakfast {
    pub key: String,
    toast: String,
}

#[derive(Debug)]
pub enum y_enum {
    a1(String),
    a2(String),
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        return Breakfast {
            key: String::from("front"),
            toast: String::from(toast),
        };
    }
}

fn service_order() {
    println!("the front_of_hose service_order");
}

