use dotenvy::from_filename;

/// Loads `.env.<NODE_ENV>` (defaults to `.env.development`).
/// Tries CWD first, then parent dir (so `cd backend && cargo run` works
/// when the env file lives at the repo root). Falls back silently to
/// process env if neither file exists (e.g. in containers).
pub fn load() {
    let env_file = format!(
        ".env.{}",
        std::env::var("NODE_ENV").unwrap_or_else(|_| "development".into())
    );

    if from_filename(&env_file).is_err() {
        let _ = from_filename(format!("../{env_file}"));
    }
}