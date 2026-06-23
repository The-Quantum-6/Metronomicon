use crate::models::permissions::Permissions;
use bitflags::bitflags;


pub fn has_perm(user_permissions: Permissions, perm: Permissions) -> bool {
    user_permissions.contains(perm)
}