use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct EventModifier: u16 {
        const ONCE = 1 << 0;
        const CAPTURE = 1 << 1;
        const PREVENT_DEFAULT = 1 << 2;
        const STOP_PROPAGATION = 1 << 3;
        const STOP_IMMEDIATE_PROPAGATION = 1 << 4;
        const NONPASSIVE = 1 << 5;
        const PASSIVE = 1 << 6;
        const TRUSTED = 1 << 7;
        const SELF = 1 << 8;
        const GLOBAL = 1 << 9;
    }
}
