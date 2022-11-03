use select::document::Document;
//use select::predicate::{Attr, Class, Name};
use select::predicate::{Attr};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::io::Write;

// TODO:
// - Follow document for each service and get a description
//   - Tools & SDKs -> https://aws.amazon.com/tools/?id=docs_gateway
//   - Aurora -> https://aws.amazon.com/rds/?id=docs_gateway
// - Build a list of other documents to add to the documentation 
//   - E.g.: where to find how to recover a backup from an RDS
// - Output to vimwiki format


fn load_content(filename: &str) -> String {
    let mut file_in = File::open(filename).unwrap();
    let mut contents = String::new();
    // TODO: error handling here
    file_in.read_to_string(&mut contents);

    return contents
}

fn from_awshtml(text: &str) -> String {
    return format!(
        "{}\n",
         text
         .replace("%3Cname%3E", "")
         .replace("%26amp%3B","&")
         .replace("%3C%2Fname%3E%0A", "")
         .replace("%3C%2Fname%3E", "")
         .replace("%3Cservice","")
         .replace("%22%3E%0A", "")
         .replace("href=%22", "")
         .replace("%20", " ")
         .replace("%28","(")
         .replace("%29",")")
         .replace("%2B","+")
         .replace("%3A",":")
         .replace("%2F","/")
         .replace("%3F","?")
         .replace("%3D","=")
         .replace("%22/","https://docs.aws.amazon.com/")
         .replace("%22https","https")
                  )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let whitelist = [
"%3Cname%3E",
"docs_gateway",
    ];
    let mut file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.csv")
        .unwrap();

    let document = Document::from(load_content("AWS_Documentation.html")
                                  .as_str()
                                  );

    for node in document.find(Attr("id", "landing-page-xml")).next() {
        for i in node.attrs() {
            if i.0 == "value" {
                for aux in i.1.split("%20%20") {
                    let mut matches = false;
                    for accepted in whitelist {
                        if aux.contains(accepted) {
                            matches = true;
                        }
                    }
                    if matches {
                        file_out.write(from_awshtml(aux).as_bytes());
                    }
                }
            }
        };
    }
    Ok(())
}
