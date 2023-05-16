use super::*;


#[test]
fn one_result() {
    let query = "crab";
    let contents = "\
Bird bird birds, 
baba- birdy bird bird!
rusty crab bird dog!
";
    assert_eq!(vec!["rusty crab bird dog!"], search(query, contents));
}


#[test]
fn case_sensitive() {
    let query = "BABA-";
    let contents = "\
Bird bird birds, 
baba- birdy bird bird!
rusty crab bird dog!
";
    assert_eq!(vec!["baba- birdy bird bird!"], search_case_insensitve(query, contents));
}
