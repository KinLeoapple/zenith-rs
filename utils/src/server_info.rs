use chrono::Local;

pub fn server_info(msg: String) -> String {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let time_block = format!("{}{}{}", "[", time, "]");
    format!("{} {}", time_block, msg)
}

pub fn display_server_info(msg: String) {
    println!("{}", server_info(msg));
}