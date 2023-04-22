// server.rs
use warp::Filter;

// Import the handlers from the separate packages.
use medical::validate_medical_data;
use authenticate::authenticate_institution;

pub async fn run_server() {
    // Define the medical validation route.
    let medical_validation_route = warp::path("medical")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(validate_medical_data);

    // Define the authentication route.
    let authentication_route = warp::path("authenticate")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(authenticate_institution);

    // Combine the routes.
    let routes = medical_validation_route.or(authentication_route);

    // Start the server.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
