use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Get {
    pub key: u64,
}

#[derive(StructOpt)]
pub struct Scan {
    #[structopt(long)]
    pub start_key: Option<u64>,
    #[structopt(long)]
    pub start_exclusive: bool,
    #[structopt(long)]
    pub end_key: Option<u64>,
    #[structopt(long)]
    pub end_inclusive: bool,
}

#[derive(StructOpt)]
pub struct Put {
    pub key: u64,
    pub value: Vec<u8>,
}

#[derive(StructOpt)]
pub struct Delete {
    pub key: u64,
}

#[derive(StructOpt)]
pub enum Opt {
    Get(Get),
    Scan(Scan),
    Put(Put),
    Delete(Delete),
}
