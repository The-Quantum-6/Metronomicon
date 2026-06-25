use bitflags::bitflags;

bitflags! {
    pub struct Permissions: i32 {
        // reading
        const READ = 1 << 1;

        // writing 
        const WRITE_TEXT = 1 << 2;
        const WRITE_FILE = 1 << 3;

        // suggest changes
        const SUGGEST_TEXT = 1 << 4;
        const SUGGEST_FILE = 1 << 5;

        // moderation
        const MODERATE_TEXT = 1 << 6;
        const MODERATE_FILE = 1 << 7;

        // page management
        const PAGE_ADMIN = 1 << 8;

        // permission transfer
        const TRANSFER_PERMS = 1 << 9;
    }
}
