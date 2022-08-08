#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use crate::brands_handlers::*;

    #[actix_web::test]
    async fn test_brand_post() {
        let app = test::init_service
            (App::new()
            .service(post_vehicle_brand)).await;
        let req = test::TestRequest::post()
            .uri("/brands")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}