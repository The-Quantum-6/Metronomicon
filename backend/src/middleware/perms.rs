use crate::{middleware::perms, models::permissions::Permissions};
use bitflags::bitflags;


pub fn has_permission(perms: Permissions, required: Permissions) -> bool {
    perms.contains(required)
}

pub fn can_transfer(perms: Permissions, requested: Permissions) -> bool {
    perms.contains(Permissions::TRANSFER_PERMS) && perms.contains(requested)
}

pub fn is_super_user(role: &Role) -> bool {
    matches!(role, Role::Root | Role::SuperUser)
}