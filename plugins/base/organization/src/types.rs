pub struct Organization {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub member_count: usize,
}

pub struct Member {
    pub id: String,
    pub org_id: String,
    pub handle: String,
    pub rank_id: String,
}
