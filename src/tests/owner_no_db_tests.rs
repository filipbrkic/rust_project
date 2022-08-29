#[cfg(test)]
mod tests {
    use crate::models::NewOwner;
    use crate::owners_handlers::*;
    use actix_web::{dev::Service, http::StatusCode, test, App};

    #[actix_web::test]
    async fn put_owner_no_db_test() {
        let app = test::init_service(App::new().service(update_owners)).await;

        let req = test::TestRequest::put()
            .uri("/owners/6")
            .set_json(&NewOwner {
                first_name: "test update".to_string(),
                last_name: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn post_owner_no_db_test() {
        let app = test::init_service(App::new().service(post_owners)).await;

        let req = test::TestRequest::post()
            .uri("/owners")
            .set_json(&NewOwner {
                first_name: "test update".to_string(),
                last_name: "test update".to_string(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn get_owners_no_db_test() {
        let app = test::init_service(App::new().service(get_owners)).await;

        let req = test::TestRequest::with_uri("/owners").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn get_owner_by_id_no_db_test() {
        let app = test::init_service(App::new().service(get_owners_by_id)).await;

        let req = test::TestRequest::with_uri("/owners/7").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn delete_owner_by_id_no_db_test() {
        let app = test::init_service(App::new().service(delete_owners)).await;

        let req = test::TestRequest::delete().uri("/owners/1").to_request();
        let res = app.call(req).await.unwrap();

        if res.status() == 200 {
            assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }
        if res.status() == 404 {
            assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[actix_web::test]
    async fn get_non_existent_owner_by_id_no_db_test() {
        let app = test::init_service(App::new().service(get_owners_by_id)).await;

        let req = test::TestRequest::with_uri("/owners/12222222").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
