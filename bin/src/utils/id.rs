use idgenerator::{IdGeneratorOptions, IdInstance};

pub fn generate_id() -> i64 {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    let _ = IdInstance::init(options).unwrap();
    let id = IdInstance::next_id();
    id
}