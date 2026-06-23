use bitflags::bitflags;

bitflags! {
    pub struct Permissions: u32 {
        // reading
        const READ = 1 << 3;

        // writing 
        const WRITE_TEXT = 1 << 4;
        const WRITE_FILE = 1 << 5;

        // suggest changes
        const SUGGEST_TEXT = 1 << 6;
        const SUGGEST_FILE = 1 << 7;

        // moderation
        const MODERATE_TEXT = 1 << 8;
        const MODERATE_FILE = 1 << 9;

        // page management
        const PAGE_ADMIN = 1 << 10;

        // permission transfer
        const TRANSFER_PERMS = 1 << 11;
    }
}
