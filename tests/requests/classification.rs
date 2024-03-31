use hagrid_api::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_create_classification() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "name": "bacia",
        });

        let res = request.post("/api/classifications").json(&payload).await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}


