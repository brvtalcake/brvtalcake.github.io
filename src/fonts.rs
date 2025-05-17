use std::{collections::BTreeMap, sync::LazyLock};

use bitflags::bitflags;

bitflags! {
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct FontStyle: u8
	{
		const NORMAL = 0;
		const BOLD   = 1 << 0;
		const ITALIC = 1 << 1;
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontId
{
	Noto,
	SofiaSemiCondensed,
	Bricolage,
	FacultyGlyphic,
	Roboto,
	Space,
	JetBrains,
	Ubuntu /* NotoSans,
	        * SofiaSansSemiCondensed,
	        * BricolageGrotesque,
	        * FacultyGlyphic,
	        * RobotoMono,
	        * SpaceMono,
	        * JetBrainsMono,
	        * Ubuntu,
	        * SpaceGrotesk */
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontKind
{
	Monospace,
	SansSerif
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontFamily
{
	id:   FontId,
	kind: FontKind
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Font
{
	name:   String,
	style:  FontStyle,
	family: FontFamily
}

// turf::style_sheet!("scss/fonts.scss");

// pub static IMPORTED_FONTS: LazyLock<BTreeMap<Font, &'static str>> =
// LazyLock::new(|| { let mut ret = BTreeMap::new();
// ret.insert(key, value)
// ret
// });
