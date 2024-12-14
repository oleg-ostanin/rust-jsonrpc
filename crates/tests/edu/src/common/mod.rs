use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub(crate) struct Types {
    pub(crate) num: i32,
    num_option: Option<i32>,
    line: String,
    pub(crate) line_option: Option<String>,
}

impl Types {
    pub(crate) fn new(num: i32) -> Self {
        let line = num.to_string();
        let line_for_opt = num.to_string();
        Types {
            num,
            num_option: Some(num),
            line,
            line_option: Some(line_for_opt),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Order {
    id: u32,
    customer_id: u32,
    products: HashMap<u32, u32>,
    comment: Option<String>,
}

impl Order {
    pub fn new(id: u32, customer_id: u32, products: HashMap<u32, u32>, comment: Option<String>) -> Self {
        Self { id, customer_id, products, comment }
    }
}

struct Customer {
    id: u32,
    phone: String,
}

const PHONE_TEMPLATE: &str = "+79112120000";

impl Customer {
    pub fn new(id: u32) -> Self {
        let id_str = id.to_string();
        let id_len = id_str.len();
        let zeros = PHONE_TEMPLATE.split_at(8 + id_len).1;
        zeros.to_string().push_str(id_str.as_str());
        let phone = PHONE_TEMPLATE.to_string().replace("0000", zeros);
        Self { id, phone }
    }
}