use std::fs;
use serde_json::Value;
use toml::value::Table;

fn check_dep(name: &str, ver: &str) -> bool
{
    let client = reqwest::blocking::Client::builder()
        .user_agent("MiseryLovesCompanyBot (github.com/cameroncros/misery_loves_company_bot)")
        .build().unwrap();

    let stats_url = format!("https://crates.io/api/v1/crates/{name}");
    let response = client.get(stats_url).send().unwrap();
    let json_str = response.text().unwrap();

    let v: Value = serde_json::from_str(json_str.as_str()).unwrap();

    let versions = v.get("versions").unwrap().as_array().unwrap();
    let mut latest_ver = "";
    let mut latest_ver_downloads = 0;
    let mut current_ver_downloads = 0;
    for version in versions {
        let vernum = version.get("num").unwrap().as_str().unwrap();
        let downloads = version.get("downloads").unwrap().as_i64().unwrap();
        if vernum == ver {
            current_ver_downloads = downloads;
        }
        if downloads > 100000 {
            latest_ver = vernum;
            latest_ver_downloads = downloads;
            break;
        }
    }

    if latest_ver != ver {
        println!("★ {name} - {ver}:  You should upgrade to {name}=={latest_ver}, it has {latest_ver_downloads} happy users!")
        return false;
    }
    else if current_ver_downloads < 100000 {
        println!("✖ {name} - {ver}:  UNSAFE LIBRARY!!1! - Would you trust only {current_ver_downloads} people?");
        return false;
    }
    else {
        println!("✔ {name} - {ver}:  SUPER SAFE LIBRARY, {current_ver_downloads} people cant be wrong");
        return true;
    }
}

pub fn check_deps() {

    let contents = fs::read_to_string("Cargo.toml")
        .expect("Should have been able to read the file");

    let config: Table = toml::from_str(contents.as_str()).unwrap();
    let dependancies = config.get("dependencies").unwrap().as_table().unwrap();
    let mut vulnerable_libs = false;
    for dep in dependancies {
        let safe = if dep.1.is_table() {
            check_dep(dep.0, dep.1.as_table().unwrap().get("version").unwrap().as_str().unwrap())
        } else if dep.1.is_str() {
            check_dep(dep.0, dep.1.as_str().unwrap())
        };
        if !safe {
            vulnerable_libs = true;
        }
    }
    if vulnerable_libs {
        println!("Unsafe libs found, aborting");
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        check_deps()
    }
}
