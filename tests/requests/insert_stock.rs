use super::prepare_data;
use hagrid_api::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_insert_new_stock() {
    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        let payload = serde_json::json!({
            "value": 1,
            "item_id": 1,
        });

        let res = request
            .post("/api/insert_stocks")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
