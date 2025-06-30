

pub struct Store{
    // This struct will hold the database connection or any other state needed for the store
}

impl Store {
    pub fn create_user(&self){

    }
    pub fn create_website(&self) -> String {
        // Here you would typically insert the website into the database and return its ID
        // For now, we will just return a dummy ID
        String::from("1")

    }
}