#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate env_logger;
extern crate kankyo;

mod commands;

use serenity::model::channel::Message;
use serenity::framework::standard::{Args, StandardFramework, CommandOptions};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

impl EventHandler for Handler {

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} has started up", ready.user.name)
    }

}

fn main() {
    
    // get bot data
    kankyo::load().expect("failed to load .env");
    println!("Loaded bot config..");

    // start logger
    env_logger::init().expect("Failed to initialize env_logger");

    let mut client =  Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler).expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("a;"))
        .before(|_ctx, msg, command_name| {
            println!("Recieved command by name {} from {}#{}", command_name, msg.author.name, msg.author.discriminator);
            true
        })
        .after(|_, _, command_name, error| {    
            match error {
                Ok(_) => println!(""),
                Err(why) => println!("Failed to process command '{}': {:?}", command_name, why)
            }
        })
        // Command reg
        .cmd("ping", commands::misc::ping)
        .command("shutdown", |c| c
            .check(owner_check)
            .cmd(commands::dev::shutdown))
    );
        
    if let Err(why) = client.start() {
        error!("Error when starting bot: {:?}", why);
    }

}

// for shutdown
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
    msg.author.id == 123188806349357062
}
