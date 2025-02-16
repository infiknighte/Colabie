use std::sync::Arc;

use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bitcode::{DecodeOwned, Encode};
use git2::{Repository, Signature};
use nanoserde::{DeRon, SerRon};
use tokio::{sync::Mutex, task::spawn_blocking};

#[derive(Clone)]
pub struct DB {
    git: Arc<Mutex<Repository>>,
}

#[derive(DeRon, SerRon)]
struct Record {
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

        Self { git }
    }

    pub async fn new_record(
        &self,
        username: String,
        pubkey: String,
    ) -> Result<git2::Oid, git2::Error> {
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

                let last_commit = repo.find_commit(
                    repo.find_branch(DEFAULT_BRANCH, git2::BranchType::Local)?
                        .into_reference()
                        .target()
                        .unwrap(),
                )?;

                // TODO: Break registrie records into subdirs like "duskyelf" -> "du/sk/duskyelf"
                let mut tree_builder = repo.treebuilder(Some(&last_commit.tree()?))?;
                tree_builder.insert(&record.username, blob, 0o100644)?;

                let tree = repo.find_tree(tree_builder.write()?)?;

                // clippy suggested code errors out - https://github.com/rust-lang/rust-clippy/issues/9794
                #[allow(clippy::let_and_return)]
                // TODO: Sign registrie git commits
                let x = repo.commit(
                    repo.find_branch(DEFAULT_BRANCH, git2::BranchType::Local)?
                        .into_reference()
                        .name(),
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
    }

    fn init_repo(path: &str) -> Result<Repository, git2::Error> {
        let repo = Repository::init_bare(path).expect("OS");
        {
            let sig = Signature::now(AUTHOR, AUTHOR)?;
            let tree = repo.find_tree(repo.treebuilder(None)?.write()?)?;
            let commit =
                repo.find_commit(repo.commit(None, &sig, &sig, "Initial Commit", &tree, &[])?)?;
            repo.branch(DEFAULT_BRANCH, &commit, false)?;
        }
        Ok(repo)
    }
}

pub struct BitCode<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for BitCode<T>
where
    T: DecodeOwned,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        Ok(Self(
            bitcode::decode(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?,
        ))
    }
}

impl<T> IntoResponse for BitCode<T>
where
    T: Encode,
{
    fn into_response(self) -> Response {
        bitcode::encode(&self.0).into_response()
    }
}
