#[cfg(test)]
mod tests {
    use crate::handlers::brands_handlers::*;
    use crate::models::NewBrand;
    use actix_web::http;
    use actix_web::{dev::Service, test, App};

    #[actix_web::test]
    async fn put_brand_no_db_test() {
        let app = test::init_service(App::new().service(update_vehicle_brands)).await;

        let req = test::TestRequest::put()
            .uri("/brands/17")
            .set_json(&NewBrand {
                name: "test update".to_string(),
                description: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn post_brand_no_db_test() {
        let app = test::init_service(App::new().service(post_vehicle_brand)).await;

        let req = test::TestRequest::post()
            .uri("/brands")
            .set_json(&NewBrand {
                name: "test".to_owned(),
                description: "test".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn get_brands_no_db_test() {
        let app = test::init_service(App::new().service(get_vehicle_brands)).await;

        let req = test::TestRequest::with_uri("/brands").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn get_brand_by_id_no_db_test() {
        let app = test::init_service(App::new().service(get_vehicle_brands_by_id)).await;

        let req = test::TestRequest::with_uri("/brands/17").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn delete_brand_by_id_no_db_test() {
        let app = test::init_service(App::new().service(delete_vehicle_brand)).await;

        let req = test::TestRequest::delete().uri("/brands/1").to_request();
        let res = app.call(req).await.unwrap();
        if res.status() == 200 {
            assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        }
        if res.status() == 404 {
            assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[actix_web::test]
    async fn get_non_existent_brand_by_id_no_db_test() {
        let app = test::init_service(App::new().service(get_vehicle_brands_by_id)).await;

        let req = test::TestRequest::with_uri("/brands/12222222").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
