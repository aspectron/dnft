use crate::prelude::*;


pub type DataVec = Vec<program::Data>;

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct DnftMeta {
    version: u32,
    identity: Pubkey,
}

#[container(Containers::Dfnt)]
pub struct Dnft {
    pub meta: RefCell<&'info mut DnftMeta>,
    pub store: SegmentStore<'info>,
    // ---
    pub data: Serialized<'info, DataVec>,

}