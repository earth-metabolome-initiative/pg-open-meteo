use pgrx::prelude::*;

::pgrx::pg_module_magic!();

#[pg_extern]
fn hello_pg_open_meteo() -> &'static str {
    "Hello, pg_open_meteo"
}

#[pg_extern]
fn strictly_positive(a: i32) -> bool {
    a > 0
}

#[pg_extern]
fn x_must_be_bigger_than_y(x: i32, y: i32) -> bool {
    if x > y {
        return true;
    }
    error!("X is smaller than Y")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pg_open_meteo() {
        assert_eq!("Hello, pg_open_meteo", crate::hello_pg_open_meteo());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
