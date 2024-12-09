/// Prints details about the repository for `did`.
pub fn describe(did: &str, pds: &str) {
    let res = super::api::repo::describe(did, pds);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
