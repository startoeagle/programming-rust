use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("serving on http://localhost:3000 ...:");
    server
        .bind("127.0.0.1:3000")
        .expect("Could not launch server")
        .run()
        .expect("Error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    let m = form.m;
    let n = form.n;

    if n == 0 || m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text")
            .body("Computing the GCD with zero is boring");
    }
    let response = format!("The gcd between {} and {} is {}", m, n, gcd(m, n));

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert!(gcd(14, 15) == 1);
}
