pub fn get_new_time() -> String {
    let now = chrono::Local::now();
    now.format("%H:%M:%S").to_string()
}
