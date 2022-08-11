#[cfg(test)]
mod tests {
    use actix_web::{test, App, http::StatusCode, dev::Service};
    use crate::brands_handlers::*;
    
    #[actix_web::test]
    async fn get_brands_test() {
        let app = test::init_service(App::new()
            .service(get_vehicle_brands)).await;

        let req = test::TestRequest::with_uri("/brands").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn post_brands_without_content_test() {
        let app = test::init_service(App::new()
            .service(post_vehicle_brand)).await;
        let req = test::TestRequest::with_uri("/brands").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}