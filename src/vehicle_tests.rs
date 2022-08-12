#[cfg(test)]
mod tests {
    use actix_web::{test, App, http::StatusCode, dev::Service};
    use crate::{handlers::brands_handlers::*, owners_handlers::*};
    
    #[actix_web::test]
    async fn get_brands_test() {
        let app = test::init_service(App::new()
            .service(get_vehicle_brands)).await;

        let req = test::TestRequest::with_uri("/brands").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_brand_by_id_test() {
        let app = test::init_service(App::new()
            .service(get_vehicle_brands_by_id)).await;

        let req = test::TestRequest::with_uri("/brands/1").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn delete_brand_by_id_test() {
        let app = test::init_service(App::new()
            .service(delete_vehicle_brand)).await;

        let req = test::TestRequest::with_uri("/brands/1").to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_non_existent_brand_by_id_test() {
        let app = test::init_service(App::new()
            .service(get_vehicle_brands_by_id)).await;

        let req = test::TestRequest::with_uri("/brands/111111").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

// -------------------------------------------------------------------------------

    #[actix_web::test]
    async fn get_owners_test() {
        let app = test::init_service(App::new()
            .service(get_owners)).await;

        let req = test::TestRequest::with_uri("/owners").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_owner_by_id_test() {
        let app = test::init_service(App::new()
            .service(get_owners_by_id)).await;

        let req = test::TestRequest::with_uri("/owners/1").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn delete_owner_by_id_test() {
        let app = test::init_service(App::new()
            .service(delete_owners)).await;

        let req = test::TestRequest::with_uri("/owners/1").to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

    }

    #[actix_web::test]
    async fn get_non_existent_owner_by_id_test() {
        let app = test::init_service(App::new()
            .service(get_owners_by_id)).await;

        let req = test::TestRequest::with_uri("/owners/111111").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}