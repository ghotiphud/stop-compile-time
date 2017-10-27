// Too many Strings...
// email_address
// zip
// state
// etc.

mod hide {
    use regex::Regex;

    #[derive(Debug)]
    pub struct Contact {
        pub name: ContactName,
        pub email: ContactEmail,
        pub address: PostalAddress,
    }

    #[derive(Debug)]
    pub struct ContactName {
        pub first_name: String,
        pub middle_name: Option<String>,
        pub last_name: String,
    }

    #[derive(Debug)]
    pub struct ContactEmail {
        pub email_address: EmailAddress,

        // true if ownership confirmed
        pub email_verified: bool,
    }

    #[derive(Debug)]
    pub struct PostalAddress {
        pub address1: String,
        pub address2: String,
        pub city: String,
        pub state: StateCode,
        pub zip: ZipCode,

        // true if validated w/ address service
        pub address_valid: bool,
    }


    #[derive(Debug)]
    /// private inner field keeps us from constructing outside this module
    pub struct EmailAddress(String);

    impl EmailAddress {
        pub fn new(string: &str) -> Result<EmailAddress, String> {
            lazy_static! {
                static ref EMAIL_REGEX: Regex = Regex::new(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap();
            }

            if EMAIL_REGEX.is_match(&string) {
                Ok(EmailAddress(String::from(string)))
            } else {
                Err("Not a valid email address".into())
            }
        }
    }


    #[derive(Debug)]
    pub struct StateCode(String);

    impl StateCode {
        const CODES: &'static [&'static str] = &["HI", "TX", "OR", "SC", "OK", "FL"];

        pub fn new(string: &str) -> Result<StateCode, String> {
            let upper = string.to_uppercase();
            if StateCode::CODES.contains(&&*upper) {
                Ok(StateCode(upper))
            } else {
                Err("Not a valid state code".into())
            }
        }
    }


    #[derive(Debug)]
    pub struct ZipCode(String);

    impl ZipCode {
        pub fn new(string: &str) -> Result<ZipCode, String> {
            lazy_static! {
                static ref ZIP_REGEX: Regex = Regex::new(r"^\d{5}(?:[-\s]\d{4})?$").unwrap();
            }

            if ZIP_REGEX.is_match(&string) {
                Ok(ZipCode(String::from(string)))
            } else {
                Err("Not a valid zipcode".into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::hide::*;

    #[test]
    fn construct_contact() {
        let _contact = Contact {
            name: ContactName {
                first_name: "Yale".to_owned(),
                middle_name: Some("M".to_owned()),
                last_name: "Cason".to_owned(),
            },
            email: ContactEmail {
                email_address: EmailAddress::new("test@example.com").unwrap(),
                email_verified: false,
            },
            address: PostalAddress {
                address1: "123 Main St.".to_owned(),
                address2: "".to_owned(),
                city: "Tampa".to_owned(),
                state: StateCode::new("FL").unwrap(),
                zip: ZipCode::new("33607").unwrap(),
                address_valid: false,
            },
        };
    }

    #[test]
    fn construct_types() {
        let email = EmailAddress::new("test@example.com");
        let state = StateCode::new("FL");
        let zip = ZipCode::new("33555");

        assert!(email.is_ok());
        assert!(state.is_ok());
        assert!(zip.is_ok());

        println!("{:?}", email.unwrap());
    }

    #[test]
    fn bad_zip() {
        let zip = ZipCode::new("abcd");

        assert!(zip.is_err(), "bad zip returns error");
    }
}