// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests

pub struct DropBomb {
    active: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { active: true }
    }

    pub fn defuse(&mut self) {
        self.active = false;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.active {
            panic!("not defused !")
        }
    }
}
