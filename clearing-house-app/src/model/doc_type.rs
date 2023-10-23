#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DocumentType {
    pub id: String,
    pub pid: String,
    pub parts: Vec<DocumentTypePart>,
}

impl DocumentType {
    #[cfg(test)]
    pub fn new(id: String, pid: String, parts: Vec<DocumentTypePart>) -> DocumentType {
        DocumentType { id, pid, parts }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DocumentTypePart {
    pub name: String,
}

impl DocumentTypePart {
    #[cfg(test)]
    pub fn new(name: String) -> DocumentTypePart {
        DocumentTypePart { name }
    }
}
