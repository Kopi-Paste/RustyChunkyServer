use axum::body::Bytes;

#[derive(Clone)] 
pub struct SavedFile {
    data : Vec<u8>,
    uploading : bool,
    mime_type : String
}

impl SavedFile {
    pub fn new(data : Bytes, mime_type : String) -> Self {
        let data_vec = data.to_vec();
        SavedFile{ data : data_vec, uploading : true, mime_type }
    }

    pub fn extend(&mut self, additional_data : Bytes) {
        self.data.extend(additional_data.to_vec());
    }

    pub fn get_mime(&self) -> &String {
        &self.mime_type
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}



