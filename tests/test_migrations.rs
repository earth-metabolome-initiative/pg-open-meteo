use diesel::{connection::SimpleConnection, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::process::Command;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

const DEFAULT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "usr";
const DATABASE_PORT: u16 = 6768;
const DATABASE_NAME: &str = "test_db";

/// Establish a connection to a postgres database.
pub fn establish_connection_to_postgres() -> Result<PgConnection, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url)
}

/// Copy a file to a docker container.
///
/// # Arguments
///
/// * `container_id` - The ID of the container.
/// * `local_path` - The path to the file on the local machine.
/// * `container_path` - The path to the file in the container.
///
async fn copy_file_to_container(container_id: &str, local_path: &str, container_path: &str) {
    // Checks whether the file exists on the local machine.
    if !std::path::Path::new(local_path).exists() {
        eprintln!("File does not exist: {}", local_path);
        std::process::exit(1);
    }

    let output = Command::new("docker")
        .args([
            "cp",
            local_path,
            &format!("{}:{}", container_id, container_path),
        ])
        .output()
        .expect("Failed to execute docker cp");

    if !output.status.success() {
        eprintln!(
            "Error copying file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Setup a docker container with a postgres database.
///
/// # Panics
///
/// * If the container cannot be started.
///
pub async fn setup_docker() -> ContainerAsync<GenericImage> {
    let container = GenericImage::new("postgres", "17")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", DATABASE_NAME)
        .with_mapped_port(DATABASE_PORT, 5432_u16.tcp())
        .start()
        .await
        .expect("Failed to start container");

    let container_id = container.id(); // Get the container ID

    copy_file_to_container(
        container_id,
        "./open_meteo_extension/usr/share/postgresql/17/extension/pg_open_meteo.control",
        "/usr/share/postgresql/17/extension/",
    )
    .await;
    copy_file_to_container(
        container_id,
        "./open_meteo_extension/usr/share/postgresql/17/extension/pg_open_meteo--0.0.0.sql",
        "/usr/share/postgresql/17/extension/",
    )
    .await;
    copy_file_to_container(
        container_id,
        "./open_meteo_extension/usr/lib/postgresql/17/lib/pg_open_meteo.so",
        "/usr/lib/postgresql/17/lib/",
    )
    .await;

    container
}

#[tokio::test]
async fn connection_and_run_migrations() {
    let container = setup_docker().await;
    let mut conn = establish_connection_to_postgres().expect("Failed to establish connection");
    conn.run_pending_migrations(DEFAULT_MIGRATIONS)
        .expect("Failed to run migrations");
    // conn.batch_execute("INSERT INTO price (value) VALUES (1);")
    //     .expect("Failed to insert value : 1.");
    // conn.batch_execute("INSERT INTO price (value) VALUES (-1);")
    //     .expect_err("Insertion of value should have failed.");

    // conn.batch_execute("INSERT INTO position (x,y) VALUES (3,2)")
    //     .expect("Failed to insert value");
    // conn.batch_execute("INSERT INTO position (x,y) VALUES (2,3)")
    //     .expect_err("Insertion should have failed");

    container.stop().await.expect("Failed to stop container");
}
