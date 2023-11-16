use rltk::{ rex::XpFile };

// TODO replace with original image
rltk::embedded_resource!(SMALL_DUNGEON, "../res/SmallDungeon_80x50.xp");

pub struct RexAssets {
    pub menu : XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../res/SmallDungeon_80x50.xp");

        RexAssets{
            menu : XpFile::from_resource("../res/SmallDungeon_80x50.xp").unwrap()
        }
    }
}