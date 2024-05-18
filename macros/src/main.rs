macro_rules! custom_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("C'mon man those can't be equal: `{:?}` IS NOT EQUAL TO `{:?}`, even a monkey can see this.",
                left_val, right_val)
                }
            }
        }
    })
}

fn main() {
    custom_eq!("one", "two");
}
