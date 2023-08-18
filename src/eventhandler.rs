use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::error;
use tracing::info;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::application::interaction::Interaction;
use serenity::builder::CreateEmbed;



pub struct Bot; // Make Bot public


impl Bot {
    pub fn create_botinfo_embed() -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.title("Bot Info");
        embed.description("This bot is made by eveeify LICENCED UNDER MIT");
        embed.color(0xff6c00);
        embed.author(|a| {
            a.name("eveeify");
            a
        });
        embed
    }


}


#[async_trait]
impl EventHandler for Bot {


    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!botinfo" {
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name("eveeify");
                        a
                    });
                    e.color(0xff6c00);
                    e.description("This is the bot info");
                    e
                })
            }).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }
    // new commands 

    //bot info command




    // - - - - - - - - - -- - - - - - - - - --  - - - - - -

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
                let content = match command.data.name.as_str() {
                    "botinfo" => Some(Self::create_botinfo_embed()),
                    _ => None,
                };
    
                if let Some(content) = content {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.add_embed(content))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
            }
        }


// - - - - - - - - - -- - - - - - - - - --  - - - - - -



    // Hello command






     // Replace This with your SERVER ID/GUILD ID. Read the Readme for a step by step guide. 




    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // guild only commands
    } 








}
