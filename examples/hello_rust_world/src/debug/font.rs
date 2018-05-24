pub struct PsvDebugScreenFont {
    pub glyphs: &'static [u8],
    pub width: i32,
    pub height: i32,
    pub first: u8,
    pub last: u8,
    pub size_w: i32,
    pub size_h: i32,
}

pub const DEBUG_FONT: PsvDebugScreenFont = PsvDebugScreenFont {
    glyphs: include_bytes!("font.bin"),
    width: 8,
    height: 8,
    first: 0,
    last: 255,
    size_w: 8,
    size_h: 8,
};
