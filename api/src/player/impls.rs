use super::*;

impl ::std::borrow::Borrow<Player> for &&Player {
    fn borrow(&self) -> &Player {
        self
    }
}
