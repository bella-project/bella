use anymap::Map;
use core::{any::Any, fmt::Debug};
use std::{collections::HashMap, fs, io::Result, sync::Arc};
use vello::peniko::{Blob, Font};
use vello::skrifa::raw::FileRef;
use vello::skrifa::FontRef;

pub type AnyMap = Map<dyn Any + Send + Sync + 'static>;

pub trait Format: Debug + Clone + Send + Sync {
    fn load_file(url: &str) -> Result<Self>;
}

#[derive(Default)]
pub struct AssetServer {
    assets: AnyMap,
}

impl AssetServer {
    pub fn load_file<T: Format + 'static>(&mut self, url: &str) -> Option<&mut T> {
        if self.assets.get::<HashMap<String, T>>().is_none() {
            self.assets.insert(HashMap::<String, T>::new());
        }

        let asset_list = match self.assets.get_mut::<HashMap<String, T>>() {
            Some(s) => s,
            None => return None,
        };

        Some(
            asset_list
                .entry(url.to_string())
                .or_insert(match T::load_file(url) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("{:?}", e);
                        return None;
                    }
                }),
        )
    }
}

// ================================
// FORMATS
// ================================

#[derive(Default, Debug, Clone)]
#[allow(dead_code)]
pub struct TextFile {
    content: String,
}

impl Format for TextFile {
    fn load_file(url: &str) -> Result<Self> {
        match fs::read_to_string(url) {
            Ok(c) => Ok(Self { content: c }),
            Err(e) => Err(e),
        }
    }
}

impl Format for Font {
    fn load_file(url: &str) -> Result<Self> {
        match fs::read(url) {
            Ok(bytes) => Ok(Font::new(Blob::new(Arc::new(bytes)), 0)),
            Err(e) => Err(e),
        }
    }
}

pub trait ToFontRef: Debug + Send + Sync {
    fn to_font_ref(&self) -> Option<FontRef<'_>>;
}

impl ToFontRef for Font {
    fn to_font_ref(&self) -> Option<FontRef<'_>> {
        let file_ref = FileRef::new(self.data.as_ref()).ok()?;
        match file_ref {
            FileRef::Font(font) => Some(font),
            FileRef::Collection(collection) => collection.get(self.index).ok(),
        }
    }
}
