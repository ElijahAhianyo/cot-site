use std::fmt::Write;
use std::sync::Mutex;

use comrak::adapters::{HeadingAdapter, HeadingMeta};
use comrak::nodes::Sourcepos;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct MdPage {
    pub link: String,
    pub title: String,
    pub status: PageStatus,
    pub content_html: String,
    pub sections: Vec<Section>,
}

impl From<&MdPage> for MdPageLink {
    fn from(value: &MdPage) -> Self {
        Self {
            link: value.link.clone(),
            title: value.title.clone(),
            status: value.status,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MdPageLink {
    pub link: String,
    pub title: String,
    pub status: PageStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    #[serde(default)]
    pub status: PageStatus,
}

/// Editorial status, independent of the documentation version.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PageStatus {
    #[default]
    Published,
    Preview,
    Proposed,
}

impl PageStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Published => "",
            Self::Preview => "Preview",
            Self::Proposed => "Proposed",
        }
    }

    pub fn notice(self) -> &'static str {
        match self {
            Self::Published => "",
            Self::Preview => {
                "Documentation draft. This page is being reviewed as part of the documentation preview. Consult the Rust API for details specific to the Cot version you use."
            }
            Self::Proposed => {
                "Proposed feature. This capability is not currently supported by Cot. The examples explain a possible design, not an available API or a release commitment."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub level: u8,
    pub title: String,
    pub anchor: String,
    pub children: Vec<Self>,
}

#[derive(Debug)]
pub struct MdPageHeadingAdapter {
    pub anchorizer: Mutex<comrak::Anchorizer>,
    pub sections: Mutex<Vec<Section>>,
}

impl HeadingAdapter for MdPageHeadingAdapter {
    fn enter(
        &self,
        output: &mut dyn Write,
        heading: &HeadingMeta,
        _sourcepos: Option<Sourcepos>,
    ) -> std::fmt::Result {
        if heading.level == 1 {
            return write!(output, "<h{}>", heading.level);
        }

        let anchor = {
            let mut anchorizer = self.anchorizer.lock().unwrap();
            anchorizer.anchorize(&heading.content)
        };

        {
            let section = Section {
                level: heading.level,
                title: heading.content.clone(),
                anchor: anchor.clone(),
                children: vec![],
            };
            let mut sections = self.sections.lock().unwrap();
            sections.push(section);
        }

        write!(
            output,
            "<h{} id=\"{}\"><a class=\"anchor-link\" href=\"#{}\" aria-label=\"Link to this section: {}\"></a>",
            heading.level, anchor, anchor, heading.content
        )
    }

    fn exit(&self, output: &mut dyn Write, heading: &HeadingMeta) -> std::fmt::Result {
        write!(output, "</h{}>", heading.level)
    }
}
