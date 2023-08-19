# Eveeify's Rust CordBot
<!-- Divider Color ff6c00 -->



![Divider 1](assets/docs/dividers.png)

<h3>In this example we will deploy a Serenity bot with Shuttle. To run this bot we need a valid Discord Token. To get started log in to the [Discord developer portal](https://discord.com/developers/applications).
</h3>


1. Click the New Application button, name your application and click Create.
2. Navigate to the Bot tab in the lefthand menu, and add a new bot.
3. On the bot page click the Reset Token button to reveal your token. Put this token in your `Secrets.toml` by making a file and using the [exsample](./Secrets.toml%20exsample). It's very important that you don't reveal your token to anyone, as it can be abused.
4. For the sake of this example, you also need to scroll down on the bot page to the Message Content Intent section and enable that option.

<h1>To add the bot to a server we need to create an invite link.</h1>


1. On your bot's application page, open the OAuth2 page via the lefthand panel.
2. Go to the URL Generator via the lefthand panel, and select the `Bot` scope as well as the `Application commands` permission in the Bot Permissions section.
3. Copy the URL, open it in your browser and select a Discord server you wish to invite the bot to.


<h1>To Run/Deploying the Discord bot</h1>

**Comming Soon**

For more information please refer to the [Discord docs](https://discord.com/developers/docs/getting-started) as well as the [Serenity repo](https://github.com/serenity-rs/serenity) for more examples.
![Divider 1](assets/docs/dividers.png)



# Publishing to github

When you publish you must follow the [MIT Licence](./LICENSE) and include it.
