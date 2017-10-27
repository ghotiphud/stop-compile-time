// atomic types - change as a unit, group concerns
// Address
// Email
// Name

/// Contact is now made up of 3 atomic pieces
struct Contact {
    name: ContactName,
    email: ContactEmail,
    address: PostalAddress,
}

struct ContactName {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

struct ContactEmail {
    email_address: String,

    email_verified: bool,
}

struct PostalAddress {
    address1: String,
    address2: String,
    city: String,
    state: String,
    zip: String,

    address_valid: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_contact() {
        let _contact = Contact {
            name: ContactName {
                first_name: "Yale".to_owned(),
                middle_name: Some("M".to_owned()),
                last_name: "Cason".to_owned(),
            },
            email: ContactEmail {
                email_address: "test@example.com".to_owned(),
                email_verified: false,
            },
            address: PostalAddress {
                address1: "123 Main St.".to_owned(),
                address2: "".to_owned(),
                city: "Tampa".to_owned(),
                state: "FL".to_owned(),
                zip: "33607".to_owned(),
                address_valid: false,
            },
        };
    }
}
