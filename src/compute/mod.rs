pub mod systems;

pub const SIZE: u32 = 1920;
pub const WORKGROUP_SIZE: u32 = 8;

#[derive(Clone)]
pub enum ComputeState {
    Loading,
    Init,
    Finished,
    Update,
}
