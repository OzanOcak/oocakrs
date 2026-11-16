pub mod delete_user_profile;
pub mod publish_post;
pub mod unpublish_post;
pub mod update_user_role;

pub(crate) fn set_published(content: &str, value: bool) -> String {
    let new_line = format!("published: {value}");
    if content.contains("published:") {
        content
            .lines()
            .map(|line| {
                if line.trim_start().starts_with("published:") {
                    new_line.clone()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        let mut out = String::new();
        let mut inserted = false;
        for line in content.lines() {
            out.push_str(line);
            out.push('\n');
            if !inserted && line.trim() == "---" {
                out.push_str(&new_line);
                out.push('\n');
                inserted = true;
            }
        }
        out
    }
}