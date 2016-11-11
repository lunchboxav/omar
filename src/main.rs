extern crate iron;
extern crate router;
extern crate rustc_serialize;

#[macro_use]
extern crate mysql;

#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use mysql as my;
use rustc_serialize::json;

#[derive(Debug, PartialEq, Eq, RustcEncodable)]
struct Produk {
    nama: String,
    harga: i32,
    deskripsi: String,
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world, "index");
    router.get("/data", get_table_data, "getTableData");

    Iron::new(router).http("localhost:3000").unwrap();
}


fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn query_produk(a: &str, b: &str, c: &str) -> Vec<Produk> {
    let pool = my::Pool::new("mysql://root:root@localhost:8889").unwrap();

    let selected_payments: Vec<Produk> =
        pool.prep_exec(format!("SELECT {}, {}, {} from omar_db.produk", a, b, c),
                       ())
            .map(|result| {
                // Memetakan hasil query ke `Vec<Produk>` menggunakan iterator di `MyQuery<row, err>`.
                // Pemanggilan pertama `map()` akan memetakan hasil ke row
                // Pemanggilan kedua `map() akan memetakan setiap row ke `Produk`
                result.map(|x| x.unwrap())
                    .map(|row| {
                        let (nama, harga, deskripsi) = my::from_row(row);
                        Produk {
                            nama: nama,
                            harga: harga,
                            deskripsi: deskripsi,
                        }
                    })
                    .collect() // Hasil query ditambahkan ke `Vec<Produk>`
            })
            .unwrap(); // Unwrap `Vec<Produk>`

    selected_payments // mengembalikan hasil berupa vector
}

fn get_table_data(_: &mut Request) -> IronResult<Response> {
    let table_data = query_produk("nama", "harga", "deskripsi");
    let s = json::encode(&table_data).unwrap();
    Ok(Response::with((status::Ok, s)))
}
