pub fn progress_string(x: f64) -> String{
    let progress_symbols =  String::from("▁▃▄▅▆▇█");
    progress_symbols.chars().nth((x*(progress_symbols.chars().count() as f64 - 1.)) as usize).unwrap_or_default().to_string()
}