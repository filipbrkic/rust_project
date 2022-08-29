#[cfg(test)]
mod tests {
    use crate::dotenv;
    use crate::handlers::brands_handlers::*;
    use crate::models::{NewBrand, NewOwner};
    use actix_web::{dev::Service, http::StatusCode, test, App};
    use actix_web::{http, web};
    use diesel::r2d2::ConnectionManager;
    use diesel::{r2d2, PgConnection};
    use std::env;

    #[actix_web::test]
    async fn get_non_existent_brand_by_id_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_vehicle_brands_by_id),
        )
        .await;

        let req = test::TestRequest::with_uri("/brands/12222222").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn invalid_post_brand_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(post_vehicle_brand),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/brands")
            .set_json(&NewOwner {
                first_name: "test".to_string(),
                last_name: "test".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn invalid_put_brand_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(update_vehicle_brands),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/brands/17")
            .set_json(&NewOwner {
                first_name: "test".to_string(),
                last_name: "test".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn invalid_id_put_brand_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(update_vehicle_brands),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/brands/12345567789")
            .set_json(&NewBrand {
                name: "test update".to_string(),
                description: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }
}
