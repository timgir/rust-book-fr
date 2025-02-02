use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();

        gestion_connexion(flux);
    }
}

fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if tampon.starts_with(get) {
        let contenu = fs::read_to_string("hello.html").unwrap();

        let reponse = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contenu.len(),
            contenu
        );

        flux.write(reponse.as_bytes()).unwrap();
        flux.flush().unwrap();
    // ANCHOR: here
    // -- partie masquée ici --
    } else {
        let ligne_statut = "HTTP/1.1 404 NOT FOUND";
        let contenu = fs::read_to_string("404.html").unwrap();

        let reponse = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            ligne_statut,
            contenu.len(),
            contenu
        );

        flux.write(reponse.as_bytes()).unwrap();
        flux.flush().unwrap();
    }
    // ANCHOR_END: here
}
