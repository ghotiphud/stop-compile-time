
struct Contact {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,

    pub email_address: String,
    // true if ownership confirmed
    pub email_verified: bool,

    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    // true if validated w/ address service
    pub address_valid: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_contact() {
        let _contact = Contact {
            first_name: "Yale".to_owned(),
            middle_name: Some("M".to_owned()),
            last_name: "Cason".to_owned(),
            email_address: "test@example.com".to_owned(),
            email_verified: false,
            address1: "123 Main St.".to_owned(),
            address2: "".to_owned(),
            city: "Tampa".to_owned(),
            state: "FL".to_owned(),
            zip: "33607".to_owned(),
            address_valid: false,
        };
    }
}
