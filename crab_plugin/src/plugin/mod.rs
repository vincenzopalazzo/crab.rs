use std::str::FromStr;

#[derive(Clone)]
pub struct CrabState;

#[derive(Debug, PartialEq)]
pub enum CrabCommand {
    Subscribe,
    Unsubscribe,
}

impl FromStr for CrabCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "subscribe" => Ok(CrabCommand::Subscribe),
            "unsubscribe" => Ok(CrabCommand::Unsubscribe),
            _ => Err(()),
        }
    }
}
