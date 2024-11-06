/// This struct represents a saved file on HTTP server
pub struct SavedFile {
    /// Bytes of the file
    data : Vec<u8>,
    /// MIME type of the file
    mime_type : String
}

impl SavedFile {
    /// Initializes a file
    pub fn new(data_vec : Vec<u8>, mime_type : String) -> Self {
        SavedFile{ data : data_vec, mime_type }
    }

    /// Adds additional bytes to the file
    pub fn extend(&mut self, additional_data : Vec<u8>) {
        self.data.extend(additional_data);
    }

    /// Returns mime type of the file
    pub fn get_mime(&self) -> &String {
        &self.mime_type
    }

    /// Returns data of the file as bytes
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}



