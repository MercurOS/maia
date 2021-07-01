use super::super::{
    EfiHandle,
    EfiStatus,
};

pub type EfiExitBootServices = extern "efiapi" fn(
    image_handle: EfiHandle,
    map_key: usize,
) -> EfiStatus;
