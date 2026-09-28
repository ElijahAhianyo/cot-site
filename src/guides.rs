use std::collections::HashMap;

use cot_site_common::md_pages::{MdPage, MdPageLink};
use cot_site_macros::md_page;

use crate::{GuideCategoryItem, GuideItem, GuideLinkCategory};

pub fn parse_guides(categories: Vec<(&'static str, Vec<GuideItem>)>) -> ParsedPagesForVersion {
    let categories_links = categories
        .iter()
        .map(|(title, items)| GuideLinkCategory {
            title,
            guides: items
                .iter()
                .map(|item| match item {
                    GuideItem::Page(page) => GuideCategoryItem::Page(MdPageLink::from(page)),
                    GuideItem::SubCategory { title, pages } => GuideCategoryItem::SubCategory {
                        title,
                        pages: pages.iter().map(MdPageLink::from).collect(),
                    },
                })
                .collect(),
        })
        .collect();

    let guide_map = categories
        .into_iter()
        .flat_map(|(_title, items)| items)
        .flat_map(|item| match item {
            GuideItem::Page(page) => vec![page],
            GuideItem::SubCategory { pages, .. } => pages,
        })
        .map(|page| (page.link.clone(), page))
        .collect();

    ParsedPagesForVersion {
        categories_links,
        guide_map,
    }
}

#[derive(Debug)]
pub(crate) struct ParsedPagesForVersion {
    pub(crate) categories_links: Vec<GuideLinkCategory>,
    pub(crate) guide_map: HashMap<String, MdPage>,
}

#[derive(Debug)]
pub(crate) struct ParsedPages {
    pub(crate) version_map: HashMap<&'static str, ParsedPagesForVersion>,
}

pub fn get_prev_next_link<'a>(
    guides: &'a [GuideLinkCategory],
    current_id: &str,
) -> (Option<&'a MdPageLink>, Option<&'a MdPageLink>) {
    // The new layout has a sequence only within an individual tutorial.
    // Historical versions retain their original guide navigation.
    let all_links: Vec<&MdPageLink> = if guides.iter().any(|category| category.title == "Tutorials")
    {
        guides
            .iter()
            .filter(|category| category.title == "Tutorials")
            .flat_map(|category| &category.guides)
            .find_map(|item| match item {
                GuideCategoryItem::SubCategory { pages, .. }
                    if pages.iter().any(|page| page.link == current_id) =>
                {
                    Some(pages.iter().collect())
                }
                _ => None,
            })
            .unwrap_or_default()
    } else {
        guides
            .iter()
            .flat_map(|category| &category.guides)
            .flat_map(|item| match item {
                GuideCategoryItem::Page(link) => vec![link],
                GuideCategoryItem::SubCategory { pages, .. } => pages.iter().collect(),
            })
            .collect()
    };

    let mut prev = None;
    let mut has_found = false;

    for link in all_links {
        if has_found {
            return (prev, Some(link));
        } else if link.link == current_id {
            has_found = true;
        } else {
            prev = Some(link);
        }
    }

    if has_found {
        (prev, None)
    } else {
        (None, None)
    }
}

pub fn get_categories(master_version: Vec<(&'static str, Vec<GuideItem>)>) -> ParsedPages {
    let version_map = HashMap::from([
        (
            "v0.1",
            vec![(
                "Getting started",
                vec![
                    GuideItem::Page(md_page!("v0.1", "introduction")),
                    GuideItem::Page(md_page!("v0.1", "templates")),
                    GuideItem::Page(md_page!("v0.1", "forms")),
                    GuideItem::Page(md_page!("v0.1", "db-models")),
                    GuideItem::Page(md_page!("v0.1", "admin-panel")),
                    GuideItem::Page(md_page!("v0.1", "static-files")),
                    GuideItem::Page(md_page!("v0.1", "error-pages")),
                    GuideItem::Page(md_page!("v0.1", "testing")),
                ],
            )],
        ),
        (
            "v0.2",
            vec![(
                "Getting started",
                vec![
                    GuideItem::Page(md_page!("v0.2", "introduction")),
                    GuideItem::Page(md_page!("v0.2", "templates")),
                    GuideItem::Page(md_page!("v0.2", "forms")),
                    GuideItem::Page(md_page!("v0.2", "db-models")),
                    GuideItem::Page(md_page!("v0.2", "admin-panel")),
                    GuideItem::Page(md_page!("v0.2", "static-files")),
                    GuideItem::Page(md_page!("v0.2", "error-pages")),
                    GuideItem::Page(md_page!("v0.2", "testing")),
                ],
            )],
        ),
        (
            "v0.3",
            vec![(
                "Getting started",
                vec![
                    GuideItem::Page(md_page!("v0.3", "introduction")),
                    GuideItem::Page(md_page!("v0.3", "templates")),
                    GuideItem::Page(md_page!("v0.3", "forms")),
                    GuideItem::Page(md_page!("v0.3", "db-models")),
                    GuideItem::Page(md_page!("v0.3", "admin-panel")),
                    GuideItem::Page(md_page!("v0.3", "static-files")),
                    GuideItem::Page(md_page!("v0.3", "error-pages")),
                    GuideItem::Page(md_page!("v0.3", "openapi")),
                    GuideItem::Page(md_page!("v0.3", "testing")),
                ],
            )],
        ),
        (
            "v0.4",
            vec![
                (
                    "Getting started",
                    vec![
                        GuideItem::Page(md_page!("v0.4", "introduction")),
                        GuideItem::Page(md_page!("v0.4", "templates")),
                        GuideItem::Page(md_page!("v0.4", "forms")),
                        GuideItem::Page(md_page!("v0.4", "db-models")),
                        GuideItem::Page(md_page!("v0.4", "admin-panel")),
                        GuideItem::Page(md_page!("v0.4", "static-files")),
                        GuideItem::Page(md_page!("v0.4", "error-pages")),
                        GuideItem::Page(md_page!("v0.4", "openapi")),
                        GuideItem::Page(md_page!("v0.4", "testing")),
                    ],
                ),
                (
                    "Upgrading",
                    vec![GuideItem::Page(md_page!("v0.4", "upgrade-guide"))],
                ),
            ],
        ),
        (
            "v0.5",
            vec![
                (
                    "Getting started",
                    vec![
                        GuideItem::Page(md_page!("v0.5", "introduction")),
                        GuideItem::Page(md_page!("v0.5", "templates")),
                        GuideItem::Page(md_page!("v0.5", "forms")),
                        GuideItem::Page(md_page!("v0.5", "db-models")),
                        GuideItem::Page(md_page!("v0.5", "admin-panel")),
                        GuideItem::Page(md_page!("v0.5", "static-files")),
                        GuideItem::Page(md_page!("v0.5", "sending-emails")),
                        GuideItem::Page(md_page!("v0.5", "caching")),
                        GuideItem::Page(md_page!("v0.5", "error-pages")),
                        GuideItem::Page(md_page!("v0.5", "openapi")),
                        GuideItem::Page(md_page!("v0.5", "testing")),
                    ],
                ),
                (
                    "Upgrading",
                    vec![GuideItem::Page(md_page!("v0.5", "upgrade-guide"))],
                ),
                (
                    "About",
                    vec![GuideItem::Page(md_page!("v0.5", "framework-comparison"))],
                ),
            ],
        ),
        // (
        //     "v0.6",
        //     vec![
        //         (
        //             "Getting started",
        //             vec![
        //                 GuideItem::Page(md_page!("v0.6", "introduction")),
        //                 GuideItem::Page(md_page!("v0.6", "templates")),
        //                 GuideItem::Page(md_page!("v0.6", "forms")),
        //                 GuideItem::Page(md_page!("v0.6", "db-models")),
        //                 GuideItem::Page(md_page!("v0.6", "admin-panel")),
        //                 GuideItem::Page(md_page!("v0.6", "static-files")),
        //                 GuideItem::Page(md_page!("v0.6", "sending-emails")),
        //                 GuideItem::Page(md_page!("v0.6", "caching")),
        //                 GuideItem::Page(md_page!("v0.6", "error-pages")),
        //                 GuideItem::Page(md_page!("v0.6", "openapi")),
        //                 GuideItem::Page(md_page!("v0.6", "testing")),
        //             ],
        //         ),
        //         (
        //             "Upgrading",
        //             vec![GuideItem::Page(md_page!("v0.6", "upgrade-guide"))],
        //         ),
        //         (
        //             "About",
        //             vec![GuideItem::Page(md_page!("v0.6", "framework-comparison"))],
        //         ),
        //     ],
        // ),
        // (
        //     "v0.7",
        //     vec![
        //         (
        //             "Getting started",
        //             vec![
        //                 GuideItem::Page(md_page!("v0.7", "introduction")),
        //                 GuideItem::Page(md_page!("v0.7", "templates")),
        //                 GuideItem::Page(md_page!("v0.7", "forms")),
        //                 GuideItem::SubCategory {
        //                     title: "Database",
        //                     pages: vec![
        //                         md_page!("v0.7", "databases/overview"),
        //                         md_page!("v0.7", "databases/queries"),
        //                     ],
        //                 },
        //                 GuideItem::Page(md_page!("v0.7", "admin-panel")),
        //                 GuideItem::Page(md_page!("v0.7", "static-files")),
        //                 GuideItem::Page(md_page!("v0.7", "sending-emails")),
        //                 GuideItem::Page(md_page!("v0.7", "caching")),
        //                 GuideItem::Page(md_page!("v0.7", "error-pages")),
        //                 GuideItem::Page(md_page!("v0.7", "openapi")),
        //                 GuideItem::Page(md_page!("v0.7", "testing")),
        //             ],
        //         ),
        //         (
        //             "Upgrading",
        //             vec![GuideItem::Page(md_page!("v0.7", "upgrade-guide"))],
        //         ),
        //         (
        //             "About",
        //             vec![GuideItem::Page(md_page!("v0.7", "framework-comparison"))],
        //         ),
        //     ],
        // ),
        ("master", master_version),
    ]);

    let version_map = version_map
        .into_iter()
        .map(|(version, pages)| (version, parse_guides(pages)))
        .collect();
    ParsedPages { version_map }
}

#[cfg(test)]
mod navigation_tests {
    use super::*;
    use cot_site_common::md_pages::PageStatus;

    fn page(link: &str) -> MdPageLink {
        MdPageLink {
            link: link.into(),
            title: link.into(),
            status: PageStatus::Published,
        }
    }

    #[test]
    fn tutorial_navigation_stays_within_its_series() {
        let categories = vec![GuideLinkCategory {
            title: "Tutorials",
            guides: vec![
                GuideCategoryItem::Page(page("tutorials")),
                GuideCategoryItem::SubCategory {
                    title: "First app",
                    pages: vec![page("one"), page("two")],
                },
                GuideCategoryItem::SubCategory {
                    title: "API",
                    pages: vec![page("api")],
                },
            ],
        }];
        let (prev, next) = get_prev_next_link(&categories, "one");
        assert!(prev.is_none());
        assert_eq!(next.unwrap().link, "two");
        let (prev, next) = get_prev_next_link(&categories, "two");
        assert_eq!(prev.unwrap().link, "one");
        assert!(next.is_none());
        assert_eq!(
            get_prev_next_link(&categories, "api").0.map(|p| &p.link),
            None
        );
        assert!(get_prev_next_link(&categories, "missing").0.is_none());
        assert!(get_prev_next_link(&categories, "tutorials").1.is_none());
    }

    #[test]
    fn explanatory_pages_have_no_artificial_reading_sequence() {
        let categories = vec![
            GuideLinkCategory {
                title: "Tutorials",
                guides: vec![],
            },
            GuideLinkCategory {
                title: "Guides",
                guides: vec![
                    GuideCategoryItem::Page(page("routing")),
                    GuideCategoryItem::Page(page("requests")),
                ],
            },
        ];
        let (prev, next) = get_prev_next_link(&categories, "routing");
        assert!(prev.is_none() && next.is_none());
        assert!(categories[1].contains("requests"));
        assert_eq!(categories[1].kind(), "Explanation");
    }

    #[test]
    fn legacy_navigation_still_crosses_groups() {
        let categories = vec![GuideLinkCategory {
            title: "Getting started",
            guides: vec![
                GuideCategoryItem::Page(page("introduction")),
                GuideCategoryItem::SubCategory {
                    title: "Database",
                    pages: vec![page("queries")],
                },
            ],
        }];
        assert_eq!(
            get_prev_next_link(&categories, "introduction")
                .1
                .unwrap()
                .link,
            "queries"
        );
        assert!(get_prev_next_link(&categories, "missing").0.is_none());
    }
}
