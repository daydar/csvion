use crate::services::data_service::*;

#[get("/data")]
pub fn csv_data() -> &'static str {
    let s = test();
    return s;

}
