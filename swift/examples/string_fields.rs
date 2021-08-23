use swift::String;
use swift_rt::metadata::Type;

fn main() {
    let fields = String::get_metadata().type_descriptor().fields().unwrap();

    println!("{:#?}", fields);
}
