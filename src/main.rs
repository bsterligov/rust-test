use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::{SwaggerUi};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
#[utoipa::path(
    get,
    path = "/",
    tag = "Root",
    responses(
        (status = 200)
    )
)]
async fn root() -> &'static str {
    "Hello, World!"
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "User",
    request_body = inline(CreateUser),
    responses(
        (status = 201, body = inline(User))
    )
)]

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };
    log::info!("User created: {:?}", user);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Debug, Serialize, ToSchema)]
struct User {
    id: u64,
    username: String,
}

#[derive(OpenApi)]
#[openapi(paths(create_user, root))]
pub struct ApiDoc;