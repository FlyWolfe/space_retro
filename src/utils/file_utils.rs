pub async fn load_string(file_name: &str, folder_path: &str) -> anyhow::Result<String> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join(folder_path)
        .join(file_name);
    println!("Path: {}", path.to_str().unwrap());
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let data = std::fs::read(path)?;

    Ok(data)
}
