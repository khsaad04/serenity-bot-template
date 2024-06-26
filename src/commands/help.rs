use serenity::all::{
    prelude::*,
    standard::{help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    Message, UserId,
};
use std::collections::HashSet;

#[help]
#[command_not_found_text = "Could not find: `{}`."]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
