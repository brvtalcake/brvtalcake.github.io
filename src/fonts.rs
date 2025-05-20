use std::{
	collections::HashSet,
	sync::{Arc, RwLock}
};

use formatx::formatx;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontWeight
{
	Custom(String),
	Light,
	Regular,
	Medium,
	Bold
}

impl ToString for FontWeight
{
	fn to_string(&self) -> String
	{
		match self
		{
			Self::Light => "light",
			Self::Regular => "regular",
			Self::Medium => "medium",
			Self::Bold => "bold",
			Self::Custom(s) => s.as_str()
		}
		.into()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontKind
{
	Monospace,
	SansSerif,
	Grotesque
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontFamily
{
	id:   FontId,
	kind: FontKind
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Font
{
	name:   String,
	family: FontFamily,
	weight: FontWeight,
	width:  Option<String>,
	italic: bool
}

impl Font
{
	pub const fn new(
		name: String,
		family: FontFamily,
		weight: FontWeight,
		width: Option<String>,
		italic: bool
	) -> Self
	{
		Self {
			name,
			family,
			weight,
			width,
			italic
		}
	}

	pub fn to_class_name(&self) -> String
	{
		let template = match self.family.id
		{
			FontId::Noto =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::SansSerif,
					"the only \"Noto\" font variant we currently use is a \"sans-serif\" one"
				);
				"noto-sans{weight}{width}{italic}"
			},
			FontId::SofiaSemiCondensed =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::SansSerif,
					"the only \"Sofia\" font variant we currently use is a \"sans-serif\", \
					 \"semi-condensed\" one"
				);
				"sofia-sans-semi-condensed{weight}{width}{italic}"
			},
			FontId::Bricolage =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::Grotesque,
					"the only \"Bricolage\" font variant we currently use is a \"sans-serif\", \
					 \"grotesque\" one"
				);
				"bricolage-grotesque{weight}{width}{italic}"
			},
			FontId::FacultyGlyphic =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::SansSerif,
					"the only \"Faculty Glyphic\" font variant we currently use is a \
					 \"sans-serif\" one"
				);
				"faculty-glyphic{weight}{width}{italic}"
			},
			FontId::Roboto =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::Monospace,
					"the only \"Roboto\" font variant we currently use is a \"monospace\"d one"
				);
				"roboto-mono{weight}{width}{italic}"
			},
			FontId::Space =>
			{
				assert!(
					matches!(self.family.kind, FontKind::Grotesque | FontKind::Monospace),
					"the only \"Space\" font variants we currently use are the \"monospace\"d one \
					 and the \"sans-serif\", \"grotesque\" one"
				);
				if matches!(self.family.kind, FontKind::Grotesque)
				{
					"space-grotesk{weight}{width}{italic}"
				}
				else
				{
					"space-mono{weight}{width}{italic}"
				}
			},
			FontId::JetBrains =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::Monospace,
					"the only \"JetBrains\" font variant we currently use is a \"monospace\"d one"
				);
				"jetbrains-mono{weight}{width}{italic}"
			},
			FontId::Ubuntu =>
			{
				assert_eq!(
					self.family.kind,
					FontKind::SansSerif,
					"the only \"Ubuntu\" font variant we currently use is a \"sans-serif\" one"
				);
				"ubuntu{weight}{width}{italic}"
			}
		};
		formatx!(
			template,
			weight = "-".to_string() + &self.weight.to_string(),
			width = self
				.width
				.clone()
				.map(|raw| "-".to_string() + &raw)
				.unwrap_or("".into()),
			italic = self.italic.then_some("-italic").unwrap_or("")
		)
		.unwrap()
	}

    pub fn full_name(&self) -> &String
    {
        &self.name
    }
}
lazy_static! {
	pub static ref REGISTERED_FONTS: RwLock<HashSet<Font>> = {
		let mut retset = HashSet::new();
		// NOTO
		retset.insert(Font::new(
			"Noto Sans Regular".into(),
			FontFamily {
				id:   FontId::Noto,
				kind: FontKind::SansSerif
			},
			FontWeight::Regular,
			None,
			false
		));
		// SOFIA
		retset.insert(Font::new(
			"Sofia Sans Semi-Condensed Regular".into(),
			FontFamily {
				id:   FontId::SofiaSemiCondensed,
				kind: FontKind::SansSerif
			},
			FontWeight::Regular,
			None,
			false
		));
		// BRICOLAGE
		retset.insert(Font::new(
			"Bricolage Grotesque Regular".into(),
			FontFamily {
				id:   FontId::Bricolage,
				kind: FontKind::Grotesque
			},
			FontWeight::Regular,
			None,
			false
		));
		// FACULTY
		retset.insert(Font::new(
			"Faculty Glyphic Regular".into(),
			FontFamily {
				id:   FontId::FacultyGlyphic,
				kind: FontKind::SansSerif
			},
			FontWeight::Regular,
			None,
			false
		));
		// ROBOTO
		retset.insert(Font::new(
			"Roboto Mono Regular".into(),
			FontFamily {
				id:   FontId::Roboto,
				kind: FontKind::Monospace
			},
			FontWeight::Regular,
			None,
			false
		));
		// SPACE
		retset.insert(Font::new(
			"Space Grotesk Regular".into(),
			FontFamily {
				id:   FontId::Space,
				kind: FontKind::Grotesque
			},
			FontWeight::Regular,
			None,
			false
		));
		retset.insert(Font::new(
			"Space Mono Regular".into(),
			FontFamily {
				id:   FontId::Space,
				kind: FontKind::Monospace
			},
			FontWeight::Regular,
			None,
			false
		));
		retset.insert(Font::new(
			"Space Mono Regular Italic".into(),
			FontFamily {
				id:   FontId::Space,
				kind: FontKind::Monospace
			},
			FontWeight::Regular,
			None,
			true
		));
		retset.insert(Font::new(
			"Space Mono Bold".into(),
			FontFamily {
				id:   FontId::Space,
				kind: FontKind::Monospace
			},
			FontWeight::Bold,
			None,
			false
		));
		retset.insert(Font::new(
			"Space Mono Bold Italic".into(),
			FontFamily {
				id:   FontId::Space,
				kind: FontKind::Monospace
			},
			FontWeight::Bold,
			None,
			true
		));
		// JETBRAINS
		retset.insert(Font::new(
			"JetBrains Mono Regular".into(),
			FontFamily {
				id:   FontId::JetBrains,
				kind: FontKind::Monospace
			},
			FontWeight::Regular,
			None,
			false
		));
		// UBUNTU
		retset.insert(Font::new(
			"Ubuntu Light".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Light,
			None,
			false
		));
		retset.insert(Font::new(
			"Ubuntu Regular".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Regular,
			None,
			false
		));
		retset.insert(Font::new(
			"Ubuntu Medium".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Medium,
			None,
			false
		));
		retset.insert(Font::new(
			"Ubuntu Bold".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Bold,
			None,
			false
		));
		retset.insert(Font::new(
			"Ubuntu Light Italic".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Light,
			None,
			true
		));
		retset.insert(Font::new(
			"Ubuntu Regular Italic".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Regular,
			None,
			true
		));
		retset.insert(Font::new(
			"Ubuntu Medium Italic".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Medium,
			None,
			true
		));
		retset.insert(Font::new(
			"Ubuntu Bold Italic".into(),
			FontFamily {
				id:   FontId::Ubuntu,
				kind: FontKind::SansSerif
			},
			FontWeight::Bold,
			None,
			true
		));
		RwLock::new(retset)
	};
}
