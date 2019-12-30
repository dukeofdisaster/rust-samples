// working with external binaries
use std::io::Write;
use std::process::{Command, Stdio};


// derive debug for printing the struct
#[derive(Debug)]
struct SearchResult {
    query: String,
    results: Vec<String>,
}

fn search_file(name: String) -> SearchResult {
    let ps_child = Command::new("find")
        .args(&[".", "-iname", &format!("{}", name)])
        .stdout(Stdio::piped())
        .output()
        .expect("Could not spawn process");

    let results = String::from_utf8_lossy(&ps_child.stdout);
    let result_rows: Vec<String> = results
        .split("\n")
        .map(|e| e.to_string())
        .filter(|s| s.len() > 1)
        .collect(); // collect it all back into the vector

    SearchResult {
        query: name,
        results: result_rows,
    }
}

// here we write to external program's stdin. 
fn process_roundtrip() -> String {
    let mut cat_child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn process");

    let stdin = cat_child.stdin.as_mut().expect("Could not attach to stdin");

    // here we write to the child's stdin process; this is possible because we
    // used as_mut() above
    stdin.write_all(b"datahere").expect("could not write to child process");
    String::from_utf8(
        cat_child
            .wait_with_output()
            .expect("Something went wrong")
            .stdout
            .as_slice()
            .iter()
            .cloned()
            .collect(),
    )
    .unwrap()
}

fn main() {
    println!("Reading from /bin/cat > {:?}", process_roundtrip());
    println!(
        "using 'find' to search for '*rs': {:?}",
        search_file("*.rs".to_owned())
    )
}