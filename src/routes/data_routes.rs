use crate::services::data_service::*;

/// Returns the csv data as a raw string
#[get("/raw/<filename>")]
pub fn csv_data_raw(filename: String) -> String {
    let result = get_raw_csv(filename);
    return result;
}
