// Any utility functions that are misc can go here

pub fn string_slice(original: String) -> Vec<String> {
    let vec_of_str: Vec<&str> = original.as_slice().split(' ').collect();
    let mut vec_of_string: Vec<String> = Vec::new();
    for x in vec_of_str.iter() {
        vec_of_string.push(x.to_string());
    }
    return vec_of_string;
}

#[test]
fn test_string_slice() {
    let expected = vec!["This".to_string(), "is".to_string(),
                        "a".to_string(), "line".to_string()];
    let input = String::from_str("This is a line");
    let output = string_slice(input);

    assert_eq!(expected, output);
}
