use swift::{Array, String};
use swift_rt::metadata::Type;

fn main() {
    let metadata = Array::<String>::get_metadata();
    let ty_name = metadata.as_metadata().name(true);

    println!("Metadata of '{}': {:#?}", ty_name, metadata);
}
