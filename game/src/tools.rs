use bitflags::bitflags;

bitflags! {
    #[derive(Default, Debug)]
    pub struct Tools: u64 {
        const PEEK_SKILL_REQ = 1 << 0;
    }
}