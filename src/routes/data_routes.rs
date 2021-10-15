use crate::services::data_service::*;

#[get("/raw")]
pub fn csv_data_raw() -> String {
    let result = get_raw_csv();
    return result;
}
