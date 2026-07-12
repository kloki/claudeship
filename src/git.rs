use git2::{Repository, StatusOptions};

pub struct GitInfo {
    pub branch: Option<String>,
    pub is_worktree: bool,
    pub untracked: usize,
    pub modified: usize,
    pub staged: usize,
    pub renamed: usize,
    pub deleted: usize,
}

impl GitInfo {
    pub fn from_dir(path: &str) -> Option<Self> {
        let repo = Repository::open(path).ok()?;

        let is_worktree = repo.is_worktree();

        let branch = repo
            .head()
            .ok()
            .and_then(|h| h.shorthand().map(String::from));

        let statuses = repo
            .statuses(Some(StatusOptions::new().include_untracked(true)))
            .ok()?;

        let mut info = Self {
            branch,
            is_worktree,
            untracked: 0,
            modified: 0,
            staged: 0,
            renamed: 0,
            deleted: 0,
        };

        for entry in statuses.iter() {
            let s = entry.status();
            if s.is_wt_new() {
                info.untracked += 1;
            }
            if s.is_wt_modified() {
                info.modified += 1;
            }
            if s.is_index_new() || s.is_index_modified() || s.is_index_typechange() {
                info.staged += 1;
            }
            if s.is_index_renamed() || s.is_wt_renamed() {
                info.renamed += 1;
            }
            if s.is_index_deleted() || s.is_wt_deleted() {
                info.deleted += 1;
            }
        }

        Some(info)
    }
}
