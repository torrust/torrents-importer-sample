#[derive(Deserialize)]
pub struct AddedTagResponse {
    pub data: String,
}

#[derive(Deserialize)]
pub struct DeletedTagResponse {
    pub data: i64,
}

#[derive(Deserialize, Debug)]
pub struct ListResponse {
    pub data: Vec<ListItem>,
}

impl ListResponse {
    /// Find the tag ID for a given tag name.
    ///
    /// # Panics
    ///
    /// Panics if the tag name is not found.
    #[must_use]
    pub fn find_tag_id(&self, tag_name: &str) -> i64 {
        self.data
            .iter()
            .find(|tag| tag.name == tag_name)
            .expect("response does not include the tag name {tag_name}")
            .tag_id
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ListItem {
    pub tag_id: i64,
    pub name: String,
}
