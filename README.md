# Cordevall for Rust

<!-- Divider Color #ff6c00 -->

<!-- Badges -->

## Features:

- Dashborad
- Command and Event handler
- Both SlashCommand And PrefixCommands

![Divider][CustomDivider]

### In this example we will deploy a Serenity bot with Shuttle. To run this bot we need a valid Discord Token. To get started log in to the [Discord developer portal][Discord App].

1. Click the New Application button, name your application and click Create.  
2. Navigate to the Bot tab in the lefthand menu, and add a new bot.  
3. On the bot page click the Reset Token button to reveal your token.  
4. Put this token in your `secrets.toml` by making a file and using the [exsample](Example.Secrets.toml).  
5. It's very important that you don't reveal your token to anyone, as it can be abused.  
6. You might also need to scroll down on the bot page to the Privileged Gateway Intents section and enable all options.  

### To add the bot to a server we need to create an invite link.

1. On your bot's application page, open the OAuth2 page via the lefthand panel.  
2. Go to the URL Generator via the lefthand panel.  
3. Select the `Bot` scope as well as the `Application commands` permission in the Bot Permissions section.  
4. Copy the URL.   
5. Open it in your browser, Select a Discord server you wish to invite the bot to.

### To Run/Deploying the Discord bot

![Divider][CustomDivider]

For more information please refer to the [Discord docs](https://discord.com/developers/docs/getting-started) as well as the [Serenity repo][Serenity] for more examples.

# Publishing to github

When you publish you must follow the [MIT Licence](./LICENSE) and include it.  
And please **Star this Template** we are people who are doing this for free, not for work it means **WE DON'T GET PAID**.

[Discord docs]: https://discord.com/developers/docs/getting-started
[Serenity]: https://github.com/serenity-rs/serenity
[Discord App]: https://discord.com/developers/applications
[CustomDivider]: assets/docs/dividers.png
