use rayon::prelude::*;

fn main() -> Result<()>{
    // ..
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(265)
        .build()
        .unwrap();

    pool.install(||{
        let scan_results: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_results {
            println!("{}:", &subdomain.domain);
            for ports in &subdomain.open.ports {
                println!("  {}", port.port);
            }
            println!("");
        }

    });
    // ...
}


