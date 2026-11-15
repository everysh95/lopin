pub mod store;
pub mod table;

pub struct OpJsonSave {
    pub path: String
}

pub fn json_save(path: &str) -> OpJsonSave {
    OpJsonSave { path: path.to_string() }
}

pub struct OpJsonLoad {
    pub path: String
}

pub fn json_load(path: &str) -> OpJsonLoad {
    OpJsonLoad { path: path.to_string() }
}

#[cfg(test)]
mod tests;