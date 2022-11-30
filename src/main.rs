mod get_vgn;

#[tokio::main]
async fn main()->Result<(), ()>{
    let abfahrten=get_vgn::vgn().await.unwrap();
    println!("{:#?}",abfahrten.Abfahrten);
    Ok(())
}
