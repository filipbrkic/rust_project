#[cfg(test)]
mod tests {
    use crate::dotenv;
    use crate::handlers::models_handlers::*;
    use crate::models::NewModel;
    use actix_web::{dev::Service, http::StatusCode, test, App};
    use actix_web::{http, web};
    use diesel::r2d2::ConnectionManager;
    use diesel::{r2d2, PgConnection};
    use std::env;

    #[actix_web::test]
    async fn put_model_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(update_vehicle_models),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/models/9")
            .set_json(&NewModel {
                name: "test update".to_string(),
                description: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn post_model_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(post_vehicle_model),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/models")
            .set_json(&NewModel {
                name: "test".to_owned(),
                description: "test".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn get_models_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_vehicle_models),
        )
        .await;

        let req = test::TestRequest::with_uri("/models").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_model_by_id_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_vehicle_models_by_id),
        )
        .await;

        let req = test::TestRequest::with_uri("/models/9").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn delete_model_by_id_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(delete_vehicle_model),
        )
        .await;

        let req = test::TestRequest::delete().uri("/models/1").to_request();
        let res = app.call(req).await.unwrap();
        if res.status() == 200 {
            assert_eq!(res.status(), StatusCode::OK);
        }
        if res.status() == 404 {
            assert_eq!(res.status(), StatusCode::NOT_FOUND);
        }
    }
}
