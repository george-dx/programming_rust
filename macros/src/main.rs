use std::collections::HashMap;

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

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
        impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

#[macro_export]
macro_rules! custom_json {
    (null) => { Json::Null };
    ([ $( $element:tt ),* ]) => {
        Json::Array(vec![ $( custom_json!($element) ),* ])
       };
    ({ $( $key:tt : $value:tt ),* }) => {
        Json::Object(Box::new(vec![
        $( ($key.to_string(), custom_json!($value)) ),*].into_iter().collect()))
    };
    ( $other:tt ) => {
        Json::from($other)
    };
}

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

fn main() {
    custom_eq!("one", "two");
    let width = 4.0;
    let desc = custom_json!({
    "width": width,
    "height": (width * 9.0 / 4.0)
    });
}

#[test]
fn json_null() {
    assert_eq!(custom_json!(null), Json::Null);
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = custom_json!(
        [
            {
                "pitch": 440.0
            }
        ]
    );
    let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
        vec![("pitch".to_string(), Json::Number(440.0))]
            .into_iter()
            .collect(),
    ))]);
}
