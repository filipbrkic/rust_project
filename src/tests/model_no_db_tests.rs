#[cfg(test)]
mod tests {
    use crate::handlers::models_handlers::*;
    use crate::models::NewModel;
    use actix_web::{dev::Service, http::StatusCode, test, App};

    #[actix_web::test]
    async fn put_model_no_db_test() {
        let app = test::init_service(App::new().service(update_vehicle_models)).await;

        let req = test::TestRequest::put()
            .uri("/models/9")
            .set_json(&NewModel {
                name: "test update".to_string(),
                description: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn post_model_no_db_test() {
        let app = test::init_service(App::new().service(post_vehicle_model)).await;

        let req = test::TestRequest::post()
            .uri("/models")
            .set_json(&NewModel {
                name: "test".to_owned(),
                description: "test".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn get_models_no_db_test() {
        let app = test::init_service(App::new().service(get_vehicle_models)).await;

        let req = test::TestRequest::with_uri("/models").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn get_model_by_id_no_db_test() {
        let app = test::init_service(App::new().service(get_vehicle_models_by_id)).await;

        let req = test::TestRequest::with_uri("/models/9").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn delete_model_by_id_no_db_test() {
        let app = test::init_service(App::new().service(delete_vehicle_model)).await;

        let req = test::TestRequest::delete().uri("/models/1").to_request();
        let res = app.call(req).await.unwrap();
        if res.status() == 200 {
            assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }
        if res.status() == 404 {
            assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[actix_web::test]
    async fn get_non_existent_model_by_id_no_db_test() {
        let app = test::init_service(App::new().service(get_vehicle_models_by_id)).await;

        let req = test::TestRequest::with_uri("/models/12222222").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
