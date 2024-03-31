use hagrid_api::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_create_items() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let classification = serde_json::json!({
            "name": "bacia",
        });

        let res_class = request.post("/api/classifications").json(&classification).await;
        println!("classification_id: {}", res_class.text());
        let roles = serde_json::json!({
            "name": "Code_128",
        });

        let res_roles = request.post("/api/roles").json(&roles).await;
        println!("role_id: {}", res_roles.text());
        let payload = serde_json::json!({
            "code": "789014",
            "role_id": 1,
            "classification_id": 1,
        });

        let res = request.post("/api/items").json(&payload).await;
        println!("items: {}", res.text());

        assert_eq!(res.status_code(), 200);
    })
    .await;
}

