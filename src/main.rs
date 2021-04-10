use longest_bytes::reader::ParsedFile;
use longest_bytes::search;

fn main() {
    // TODO: replace with reading all files from root that start with 'sample'
    let parsed_files: Vec<ParsedFile> = vec![
        "sample.1",
        "sample.2",
        "sample.3",
        "sample.4",
        "sample.5",
        "sample.6",
        "sample.7",
        "sample.8",
        "sample.9",
        "sample.10",
        ].iter().map(|s| ParsedFile::new(s, "./")).collect();

    let res = search(&parsed_files[0], &parsed_files[1]);
    println!("result: {:?}", res);
}
