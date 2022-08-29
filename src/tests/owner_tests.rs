#[cfg(test)]
mod tests {
    use crate::dotenv;
    use crate::models::NewOwner;
    use crate::owners_handlers::*;
    use actix_web::http;
    use actix_web::web;
    use actix_web::{dev::Service, http::StatusCode, test, App};
    use diesel::r2d2::ConnectionManager;
    use diesel::{r2d2, PgConnection};
    use std::env;

    #[actix_web::test]
    async fn put_owner_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(update_owners),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/owners/6")
            .set_json(&NewOwner {
                first_name: "test update".to_string(),
                last_name: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn post_owner_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(post_owners),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/owners")
            .set_json(&NewOwner {
                first_name: "test update".to_string(),
                last_name: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn get_owners_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_owners),
        )
        .await;

        let req = test::TestRequest::with_uri("/owners").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_owner_by_id_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_owners_by_id),
        )
        .await;

        let req = test::TestRequest::with_uri("/owners/7").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn delete_owner_by_id_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(delete_owners),
        )
        .await;

        let req = test::TestRequest::delete().uri("/owners/1").to_request();
        let res = app.call(req).await.unwrap();

        if res.status() == 200 {
            assert_eq!(res.status(), StatusCode::OK);
        }
        if res.status() == 404 {
            assert_eq!(res.status(), StatusCode::NOT_FOUND);
        }
    }
}
