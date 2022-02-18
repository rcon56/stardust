use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginator {
    pub num: usize,
    pub has_prev: bool,
    pub has_next: bool,
    pub prev_url: String,
    pub next_url: String,
    pub prev_name: Option<String>,
    pub next_name: Option<String>,
}

impl Paginator {
    pub fn init(num: usize) -> Paginator {
        Paginator {
            num: num,
            has_next: false,
            has_prev: false,
            next_url: "".to_string(),
            prev_url: "".to_string(),
            prev_name: None,
            next_name: None,
        }
    }

    pub fn new(num: usize, prev_url: Option<String>, next_url: Option<String>) -> Paginator {
        Paginator { 
            num: num, 
            has_prev: prev_url.is_some(), 
            has_next: next_url.is_some(), 
            prev_url: prev_url.unwrap_or("".to_string()), 
            next_url: next_url.unwrap_or("".to_string()), 
            prev_name: None,
            next_name: None,
        }
    }

    pub fn new_at(num: usize, idx: i32, base_url: &str) -> Paginator {
        Paginator::new(num, Paginator::gen_url(num, idx-1, base_url), Paginator::gen_url(num, idx+1, base_url))
    }

    pub fn new_for_post(num: usize, prev: Option<(String, String)>, next: Option<(String, String)>) -> Paginator {
        let (has_prev, has_next) = (prev.is_some(), next.is_some());
        let (prev_url, prev_name) = prev.unwrap_or(("".to_string(), "".to_string()));
        let (next_url, next_name) = next.unwrap_or(("".to_string(), "".to_string()));
        Paginator { 
            num, 
            has_prev, has_next, 
            prev_url, next_url, 
            prev_name: Some(prev_name),
            next_name: Some(next_name),
        }
    }

    pub fn gen_url(n: usize, i: i32, base_url: &str) -> Option<String> {
        if i < 0 || i >= n as i32 { return None; }
        if i == 0 {Some(base_url.to_string())} else {Some(format!("{}/page/{}", base_url, i+1))}
    }
}