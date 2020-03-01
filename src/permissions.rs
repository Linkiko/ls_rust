use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

extern crate libc;

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

enum PermissionEntity {
    User,
    Group,
    Other,
}

#[derive(Debug)]
struct Triplet {
    r: bool,
    w: bool,
    x: bool,
}

impl Triplet {
    fn from_permission(mode: u32, permissions_entity: PermissionEntity) -> Triplet {
        let perm_entity_mask = match permissions_entity {
            PermissionEntity::User => (S_IRUSR, S_IWUSR, S_IXUSR),
            PermissionEntity::Group => (S_IRGRP, S_IWGRP, S_IXGRP),
            PermissionEntity::Other => (S_IROTH, S_IWOTH, S_IXOTH),
        };
        Triplet {
            r: mode & perm_entity_mask.0 != 0,
            w: mode & perm_entity_mask.1 != 0,
            x: mode & perm_entity_mask.2 != 0,
        }
    }

    fn to_str(&self) -> String {
        let read_str = if self.r == true { "r" } else { "-" };
        let write_str = if self.w == true { "w" } else { "-" };
        let execute_str = if self.x == true { "x" } else { "-" };
        String::from(read_str) + write_str + execute_str
    }

    fn new(read: bool, write: bool, execute: bool) -> Triplet {
        Triplet {
            r: read,
            w: write,
            x: execute,
        }
    }
}

#[derive(Debug)]
pub struct UnixPermissions {
    user: Triplet,
    group: Triplet,
    other: Triplet,
}

impl UnixPermissions {
    pub fn from_path(path: &PathBuf) -> UnixPermissions {
        let meta: fs::Metadata = fs::metadata(path.to_str().unwrap()).unwrap();
        let mode: u32 = meta.permissions().mode();
        UnixPermissions {
            user: Triplet::from_permission(mode, PermissionEntity::User),
            group: Triplet::from_permission(mode, PermissionEntity::Group),
            other: Triplet::from_permission(mode, PermissionEntity::Other),
        }
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}{}{}",
            self.user.to_str(),
            self.group.to_str(),
            self.other.to_str()
        )
    }
}
