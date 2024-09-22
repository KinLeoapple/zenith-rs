pub fn banner() -> String {
    let banner = r"
 ______           _ _   _
|___  /          (_| | | |
   / /  ___ _ __  _| |_| |__
  / /  / _ | '_ \| | __| '_ \
./ /__|  __| | | | | |_| | | |
\_____/\___|_| |_|_|\__|_| |_|";
    let version = format!("{}{}", "v", env!("CARGO_PKG_VERSION"));
    let str = format!("{} {}", banner, version);
    str.to_string()
}