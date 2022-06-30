use super::*;

impl ::std::borrow::Borrow<Clan> for &&Clan {
    fn borrow(&self) -> &Clan {
        self
    }
}
