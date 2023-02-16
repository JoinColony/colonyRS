use ethers::contract::Abigen;
use eyre::Report;

fn main() -> Result<(), Report> {
    Abigen::new("ColonyNetwork", "./abis/dist/versions/glwss2/ColonyNetwork.json")?
        .generate()?
        .write_to_file("./src/contracts/colony_network.rs")?;
    Abigen::new("Colony", "./abis/dist/versions/glwss2/IColony.json")?
        .generate()?
        .write_to_file("./src/contracts/colony.rs")?;
    Ok(())
}
