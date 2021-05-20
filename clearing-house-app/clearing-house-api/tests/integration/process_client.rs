/// Testcase: Standard case: Create process
#[test]
fn test_create_process() -> Result<()>{
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let pid = String::from("test_create_process_pid");

    // run test
    assert!(doc_api.create_process(&TOKEN.to_string(), &process)?);

    // clean up
    assert!(doc_api.delete_process(&TOKEN.to_string(), &pid)?);

    Ok(())
}


/// Testcase: Create process with already existing pid
#[test]
fn test_create_process_pid_exists() -> Result<()>{
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let pid = String::from("test_create_process_pid_exists_pid");
    let process = Process::new(pid);
    assert!(doc_api.create_process(&TOKEN.to_string(), &process)?);

    // run test
    assert_eq!(doc_api.create_process(&TOKEN.to_string(), &process)?, false);

    // clean up
    assert!(doc_api.delete_process(&TOKEN.to_string(), &pid)?);

    Ok(())
}


/// Testcase: Delete process
#[test]
fn test_delete_process() -> Result<()>{
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let pid = String::from("test_delete_process_pid");
    let process = Process::new(pid.clone());
    assert!(doc_api.create_process(&TOKEN.to_string(), &process)?);

    // run test
    assert!(doc_api.delete_process(&TOKEN.to_string(), &pid)?);

    Ok(())
}

/// Testcase: Delete process pid does not exist
#[test]
fn test_delete_process_pid_does_not_exist() -> Result<()>{
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let pid = String::from("test_delete_process_pid_does_not_exist_pid");
    let wrong_pid = String::from("test_delete_process_pid_does_not_exist_pid2");
    let process = Process::new(pid);
    assert!(doc_api.create_process(&TOKEN.to_string(), &process)?);

    // run test
    assert_eq!(doc_api.delete_process(&TOKEN.to_string(), &wrong_pid)?, false);

    // clean up
    assert!(doc_api.delete_process(&TOKEN.to_string(), &pid)?);

    Ok(())
}

// Testcase: Standard case: Get process
// Testcase: Get process with not existing pid