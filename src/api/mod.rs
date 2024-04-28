pub mod claim;
pub mod deposit;
pub mod list;
pub mod register;
pub mod withdraw;

pub use claim::*;
pub use deposit::*;
pub use list::*;
pub use register::*;
pub use withdraw::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurrowApiResponse<T> {
    code: String,
    msg: String,
    data: T,
}
