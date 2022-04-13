use poem::test::TestClient;

#[tokio::test]
async fn test_helloworld_handler() {
    let cli = TestClient::new(crate::router::app_router());

    //Test case with name.
    let test_case_1 = cli.get("/helloworld").query("name", &"AlfredENeumann").send().await;
    test_case_1.assert_status_is_ok();
    test_case_1.assert_text("Hello Alfred E Neumann").await;

    //Test case with empty name.
    let test_case_2 = cli.get("/helloworld").query("name", &"").send().await;
    test_case_2.assert_status_is_ok();
    test_case_2.assert_text("Hello Stranger").await;

    //Test case without name query param.
    let test_case_2 = cli.get("/helloworld").send().await;
    test_case_2.assert_status_is_ok();
    test_case_2.assert_text("Hello Stranger").await;
}


#[tokio::test]
async fn test_versionz_handler() {
    let cli = TestClient::new(crate::router::app_router());

    //Test case 
    let test_case = cli.get("/versionz").send().await;
    test_case.assert_status_is_ok();
    test_case.assert_content_type("application/json; charset=utf-8");
    let version_json = test_case.json().await;
    let json_value = version_json.value();
    json_value.object().get("name").assert_string("code-challenge");
    json_value.object().get("rev").assert_string(env!("GIT_SHA"));
}