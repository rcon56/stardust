use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginator {
    pub num: usize,
    pub has_prev: bool,
    pub has_next: bool,
    pub prev_url: String,
    pub next_url: String,
}

impl Paginator {
    pub fn init(num: usize) -> Paginator {
        Paginator {
            num: num,
            has_next: false,
            has_prev: false,
            next_url: "".to_string(),
            prev_url: "".to_string(),
        }
    }

    pub fn new(num: usize, prev_url: Option<String>, next_url: Option<String>) -> Paginator {
        Paginator { 
            num: num, 
            has_prev: prev_url.is_some(), 
            has_next: next_url.is_some(), 
            prev_url: prev_url.unwrap_or("".to_string()), 
            next_url: next_url.unwrap_or("".to_string()), 
        }
    }

    pub fn new_at(num: usize, idx: i32, base_url: &str) -> Paginator {
        Paginator::new(num, Paginator::gen_url(num, idx-1, base_url), Paginator::gen_url(num, idx+1, base_url))
    }

    pub fn gen_url(n: usize, i: i32, base_url: &str) -> Option<String> {
        if i < 0 || i >= n as i32 { return None; }
        if i == 0 {Some(base_url.to_string())} else {Some(format!("{}/page/{}", base_url, i+1))}
    }
}