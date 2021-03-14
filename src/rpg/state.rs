#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Alive { life: u16 },
    Dead,
}
