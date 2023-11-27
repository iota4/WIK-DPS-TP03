use warp::{Filter};

#[tokio::main]
async fn main() {
    // Crée un filtre pour gérer les requêtes GET à l'adresse /ping
    let get_request = warp::path!("ping")
        .and(warp::get())
        .map(|| {
            println!("Got GET ping");
            let response = warp::reply::json(&"It works");
            warp::reply::with_status(response, warp::http::StatusCode::OK)
        });

    // Crée un filtre pour gérer les requêtes POST à l'adresse /ping
    let post_request = warp::path!("ping")
        .and(warp::any())
        .map(|| warp::reply::with_status(warp::reply::html(""), warp::http::StatusCode::NOT_FOUND));

    // Combine les filtres pour gérer les deux types de requêtes
    let request = get_request.or(post_request);

    // Crée un serveur HTTP qui écoute sur l'adresse 127.0.0.1:8080
    warp::serve(request)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
