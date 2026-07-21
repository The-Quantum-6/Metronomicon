use crate::models::{permissions::Permissions, user::UserRole};
use bitflags::bitflags;

pub fn has_permission(perms: Permissions, required: Permissions) -> bool {
    perms.contains(required)
}

pub fn can_transfer(perms: Permissions, requested: Permissions) -> bool {
    perms.contains(Permissions::TRANSFER_PERMS) && perms.contains(requested)
}

pub fn add_permission(perms: &mut Permissions, new_perm: Permissions) {
    perms.insert(new_perm);
}

pub fn remove_permission(perms: &mut Permissions, perm_to_remove: Permissions) {
    perms.remove(perm_to_remove);
}

pub fn is_super_user(role: &UserRole) -> bool {
    matches!(role, UserRole::Root | UserRole::Admin)
}
