use structopt::StructOpt;

use client::Opt;

fn main() -> client::Result<()> {
    match Opt::from_args() {
        Opt::Get(get) => println!("{:?}", client::get(get.key)?),
        Opt::Scan(scan) => {
            let start = scan.start_key.map(|s| (s, scan.start_exclusive));
            let end = scan.end_key.map(|e| (e, scan.end_inclusive));
            println!("{:?}", client::scan(start, end)?);
        }
        Opt::Put(put) => client::put(put.key, &put.value)?,
        Opt::Delete(delete) => client::delete(delete.key)?,
    }
    Ok(())
}
