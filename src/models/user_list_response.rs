use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Users {
    pub id: String,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub data: Vec<Users>,
    pub meta: Meta,
    pub _link: Link,
}

#[derive(Debug, Serialize)]
pub struct Meta {
    pub offset: u64,
    pub limit: i64,
    pub total_results: u64,
    pub search_criteria: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Link {
    pub first: LinkHref,
    pub last: LinkHref,
    pub previous: Option<LinkHref>,
    pub next: Option<LinkHref>,
    pub self_link: LinkHref,
}

#[derive(Debug, Serialize)]
pub struct LinkHref {
    pub href: String,
}
