use std::sync::Arc;

use git2::{Repository, Signature};
use nanoserde::{DeRon, SerRon};
use tokio::{sync::Mutex, task::spawn_blocking};

use crate::erout;

#[derive(Clone)]
pub struct DB {
    git: Arc<Mutex<Repository>>,
}

#[derive(DeRon, SerRon)]
pub struct Record {
    username: String,
    pubkey: String,
}

const AUTHOR: &str = "registrie";
const DEFAULT_BRANCH: &str = "main";

impl DB {
    // This function blocks on fs operations
    // That's fine as it's called once at the very start
    pub fn get_or_create(path: &str) -> Self {
        let git = Arc::new(Mutex::new(
            Repository::open_bare(path).unwrap_or_else(|_| DB::init_repo(path).unwrap()),
        ));

        tracing::info!("openned git database repo");
        Self { git }
    }

    pub async fn new_record(&self, username: String, pubkey: String) -> git2::Oid {
        let record = Record { username, pubkey };

        let db = self.clone();
        let handle = tokio::runtime::Handle::current();

        spawn_blocking(move || {
            let blob = handle
                .block_on(async { db.git.lock().await })
                .blob(record.serialize_ron().as_bytes())?;

            {
                let repo = handle.block_on(async { db.git.lock().await });
                let sig = Signature::now(AUTHOR, AUTHOR)?;

                let reference = repo
                    .find_branch(DEFAULT_BRANCH, git2::BranchType::Local)
                    .expect("Defautl Branch")
                    .into_reference();

                let last_commit = repo
                    .find_commit(reference.target().unwrap())
                    .expect("head commit");

                // TODO: Break registrie records into subdirs like "duskyelf" -> "du/sk/duskyelf"
                // labels: enhancement, good first issue
                // Issue URL: https://github.com/Colabie/Colabie/issues/8
                let mut tree_builder = repo
                    .treebuilder(Some(&last_commit.tree()?))
                    .expect("Tree building");

                tree_builder.insert(&record.username, blob, 0o100644)?;

                let tree = repo.find_tree(tree_builder.write()?)?;

                // clippy suggested code errors out - https://github.com/rust-lang/rust-clippy/issues/9794
                #[allow(clippy::let_and_return)]
                // TODO: Sign registrie's git commits
                // labels: enhancement
                // Issue URL: https://github.com/Colabie/Colabie/issues/7
                let x = repo.commit(
                    reference.name(),
                    &sig,
                    &sig,
                    &format!("Register: {}", record.username),
                    &tree,
                    &[&last_commit],
                );
                x
            }
        })
        .await
        .unwrap()
        .unwrap()
    }

    pub async fn fetch_record(&self, username: String) -> Option<Record> {
        let handle = tokio::runtime::Handle::current();

        let db = self.clone();
        spawn_blocking(move || {
            let commit_id = handle
                .block_on(db.git.lock())
                .find_branch(DEFAULT_BRANCH, git2::BranchType::Local)
                .expect("Defautl Branch")
                .into_reference()
                .target()
                .unwrap();

            let blob_id = handle
                .block_on(db.git.lock())
                .find_commit(commit_id)
                .expect("head commit")
                .tree()
                .unwrap()
                .get_name(&username)?
                .id();

            let record = DeRon::deserialize_ron(
                std::str::from_utf8(
                    handle
                        .block_on(db.git.lock())
                        .find_blob(blob_id)
                        .unwrap()
                        .content(),
                )
                .expect("Utf-8 str"),
            )
            .expect("Valid record");

            Some(record)
        })
        .await
        .unwrap()
    }

    fn init_repo(path: &str) -> Result<Repository, git2::Error> {
        tracing::info!("initializing new git database repo");
        let repo = Repository::init_bare(path).expect("OS");
        {
            let sig = Signature::now(AUTHOR, AUTHOR)?;
            let tree = repo.find_tree(repo.treebuilder(None)?.write()?)?;
            let commit = repo.find_commit(erout!(repo.commit(
                None,
                &sig,
                &sig,
                "Initial Commit",
                &tree,
                &[]
            )))?;

            repo.branch(DEFAULT_BRANCH, &commit, false)
                .expect("Default branch");
        }
        Ok(repo)
    }
}

#[cfg(test)]
mod db_tests {
    use tokio::task::spawn_blocking;

    use super::DB;
    use std::fs;

    #[tokio::test]
    async fn new_record() {
        let path = rand::random::<u64>().to_string();
        let db = {
            let path = path.clone();
            spawn_blocking(move || DB::get_or_create(&path))
                .await
                .unwrap()
        };

        let username = "DuskyElf".to_string();
        let pubkey = "this is a test public key".to_string();

        db.new_record(username.clone(), pubkey.clone()).await;

        let record = db
            .fetch_record(username.clone())
            .await
            .expect("implementation");

        assert_eq!(username, record.username);
        assert_eq!(pubkey, record.pubkey);

        fs::remove_dir_all(path).unwrap();
    }
}
