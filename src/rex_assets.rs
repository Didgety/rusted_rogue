use rltk::{ rex::XpFile };

// TODO replace with original image
rltk::embedded_resource!(SMALL_DUNGEON, "../res/SmallDungeon_80x50.xp");
// TODO replace with new demo maps
rltk::embedded_resource!(WFC_DEMO_IMAGE1, "../res/wfc-demo1.xp");
rltk::embedded_resource!(WFC_POPULATED, "../res/wfc-populated.xp");

pub struct RexAssets {
    pub menu : XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../res/SmallDungeon_80x50.xp");
        rltk::link_resource!(WFC_DEMO_IMAGE1, "../../resources/wfc-demo1.xp");
        rltk::link_resource!(WFC_POPULATED, "../res/wfc-populated.xp");

        RexAssets{
            menu : XpFile::from_resource("../res/SmallDungeon_80x50.xp").unwrap()
        }
    }
}