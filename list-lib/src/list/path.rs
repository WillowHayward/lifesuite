pub enum PathType {
    Numbered, // Automatically numbered
    Named, // Manually named
}
pub struct Path {
    pub path_type: PathType,
}
