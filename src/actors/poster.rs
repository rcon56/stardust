use std::fs;

pub struct Poster {
}


impl Poster {

    pub fn read_md_in_dir(post_dir: &str) -> Vec<(String, String)> {

        Poster::read_with_suffix_in_dir(post_dir, ".md")
            .into_iter()
            .filter_map(|s| {
                let segments: Vec<&str> = s.splitn(3, "---\n").collect();
                if segments.len() == 3 {
                    Some((format!("---\n{}", segments[1]), segments[2].to_string()))
                } else { None }
            })
            .collect()
    }

    fn read_with_suffix_in_dir(dir: &str, suffix: &str) -> Vec<String> {

        walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().display().to_string().ends_with(suffix))
            .filter_map(|e| fs::read_to_string(e.path()).ok())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_test() {
        let test_content = "---\nauthor:lds56\ndate: Feb 9 2022\n---# Title\nThis is Content";
    }
}