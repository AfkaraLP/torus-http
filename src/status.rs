// TODO: yeah fill this out should not take long but too lazy rn
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum HttpStatus {
    Ok = 200,
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        match self {
            HttpStatus::Ok => "200 OK".into(),
        }
    }
}
