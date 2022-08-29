pub mod brand_tests;
pub mod model_tests;
pub mod owner_tests;

pub mod brand_no_db_tests;
pub mod model_no_db_tests;
pub mod owner_no_db_tests;

pub mod invalid_brand_tests;
pub mod invalid_model_tests;
pub mod invalid_owner_tests;

pub use self::brand_tests::*;
pub use self::model_tests::*;
pub use self::owner_tests::*;

pub use self::brand_no_db_tests::*;
pub use self::model_no_db_tests::*;
pub use self::owner_no_db_tests::*;

pub use self::invalid_brand_tests::*;
pub use self::invalid_model_tests::*;
pub use self::invalid_owner_tests::*;
