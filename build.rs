use ethers::contract::Abigen;
use eyre::Report;

fn main() -> Result<(), Report> {
    Abigen::new(
        "ColonyNetwork",
        "./abis/dist/versions/glwss3/IColonyNetwork.json",
    )?
    .generate()?
    .write_to_file("./src/contracts/colony_network.rs")?;
    Abigen::new("Colony", "./abis/dist/versions/glwss3/IColony.json")?
        .generate()?
        .write_to_file("./src/contracts/colony.rs")?;
    Abigen::new("tokenERC20", "./abis/dist/tokens/TokenERC20.json")?
        .generate()?
        .write_to_file("./src/contracts/tokens_erc20.rs")?;
    Ok(())
}
