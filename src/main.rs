
mod payload;
mod unique_ids;
mod echo;

use echo::EchoNode;
use payload::Message;
use anyhow::Context;

fn main() -> anyhow::Result<()>{
   let stdin = std::io::stdin().lock();
   let mut stdout = std::io::stdout().lock();

   let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

   let mut state = EchoNode{
      messages: Vec::new()
   };

   for input in inputs{
    let input = input.context("STDIN input couldn't be deserialized")?;
    state.step(input, &mut stdout).context("Node step function failed")?;
   }

   Ok(())
}
