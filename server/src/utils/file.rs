use crate::stats::ServerStats;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use super::GeneralErrors;

pub fn write_server_data_to_file(server_stats: &ServerStats) -> Result<(), GeneralErrors> {
    let json_data = serde_json::to_string_pretty(server_stats)
        .map_err(|_| GeneralErrors::SerializationError)?;
    let mut file =
        File::create("./stats/server_stats.json").map_err(|_| GeneralErrors::CreateFileError)?;
    file.write_all(json_data.as_bytes())
        .map_err(|_| GeneralErrors::WriteFileError)?;
    Ok(())
}

pub fn read_from_file() -> Result<ServerStats, GeneralErrors> {
    let mut file =
        File::open("./stats/server_stats.json").map_err(|_| GeneralErrors::ReadFileError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| GeneralErrors::ReadFileError)?;
    let server_stats: ServerStats =
        serde_json::from_str(&contents).map_err(|_| GeneralErrors::DeserializationError)?;
    remove_file()?;
    Ok(server_stats)
}

fn remove_file() -> Result<(), GeneralErrors> {
    fs::remove_file("./stats/server_stats.json").map_err(|_| GeneralErrors::DeleteFileError)?;
    Ok(())
}
