use crate::{consts, utils, Context};

use color_eyre::eyre::{Context as _, Result};

/// Ask teawie a question!
#[poise::command(prefix_command, slash_command)]
pub async fn ask(
	ctx: Context<'_>,
	#[rename = "question"]
	#[description = "The question you want to ask teawie"]
	_question: String,
) -> Result<()> {
	let resp = utils::random_choice(consts::RESPONSES)
		.wrap_err("Couldn't choose from random responses!")?;

	ctx.say(resp).await?;
	Ok(())
}
