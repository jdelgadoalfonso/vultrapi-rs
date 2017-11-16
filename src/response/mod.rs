pub use self::account::Account;
pub use self::application::{Application, Applications};
pub use self::auth::Auth;
pub use self::backup::{Backup, Backups};
pub use self::header::HeaderOnly;
pub use self::operating_system::{OperatingSystem, OperatingSystems};
pub use self::plan::{Plan, Plans};
pub use self::region::{Region, Regions};
pub use self::schedule::Schedule;
pub use self::server::{CreatedServer, Server, Servers};
pub use self::snapshot::{CreatedSnapshot, Snapshot, Snapshots};
pub use self::named_response::NamedResponse;

mod account;
mod application;
mod auth;
mod backup;
mod header;
mod operating_system;
mod plan;
mod region;
mod schedule;
mod server;
mod snapshot;
mod named_response;

pub trait NotArray {}
