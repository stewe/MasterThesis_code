pub struct Sensor {
    endpoint: String,
    filters: Vec<String>,
    // max_ages: Vec<ui32> better use a map?
}

impl Sensor {
    pub fn new (endpoint: &str, filters : &[&str]) -> Sensor {
        Sensor {
            endpoint : endpoint.to_string(),
            filters : to_string_vec(filters),
        }
    }
}

fn to_string_vec (str_arr : &[&str]) -> Vec<String>{
    let mut res: Vec<String> = Vec::with_capacity(str_arr.len());
    res.push("test".to_string());
    for s in str_arr {
        res.push(s.to_string());
    }
    res
}
