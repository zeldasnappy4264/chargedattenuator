use std::collections::HashMap;
const TAG: &str = "alloc-trace-24a008";
fn collect_data() -> HashMap<String, String> { let mut m = HashMap::new(); m.insert("tag".into(), TAG.into()); m.insert("status".into(), "active".into()); m.insert("env".into(), std::env::var("ENV").unwrap_or_else(|_| "prod".into())); m }
fn main() { println!("[{}] Collecting data...", TAG); let data = collect_data(); for (k, v) in &data { println!("  {}: {}", k, v); } println!("[{}] Done.", TAG); }
