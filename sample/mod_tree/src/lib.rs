mod my_module_1;
use my_module_1::func as print1;
mod my_module_2;
use my_module_2::func as print2;

pub fn print() {
    print1();
    print2();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn test_print() {
    //     print();
    // }
}
