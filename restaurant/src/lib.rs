
pub mod front_of_house;

pub fn service_order(a :i32) -> i32 {
    println!("the front_of_hose service_order");
    a+1
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test() {
    assert_eq!(4,service_order(3));
    }

    #[test]
    fn aa() {
        assert_eq!(dsd(1),3)
    }

    fn dsd (a:u32) ->u32 {
        a+1
    }
}
