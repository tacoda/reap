#![deny(warnings)]
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "reap")]
struct Reap {
    #[structopt()]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let reap = Reap::from_args();

    eprintln!("Fetching {:?}...", reap.url);

    // reqwest::get() is a convenience function.
    //
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let res = reqwest::get(reap.url).await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{}", body);

    Ok(())
}
