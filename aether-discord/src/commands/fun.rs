// =============================================================
//  aether-discord — src/commands/fun.rs
//  Fun: 8ball, dice, roast, compliment, coinflip, rps
// =============================================================

use rand::Rng;
use serenity::all::*;

pub fn register() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("8ball")
            .description("Ask the magic 8ball a question")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "question", "Your question").required(true)),

        CreateCommand::new("dice")
            .description("Roll a dice")
            .add_option(
                CreateCommandOption::new(CommandOptionType::Integer, "sides", "Number of sides (default: 6)")
                    .required(false),
            ),

        CreateCommand::new("roast")
            .description("Roast a user")
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to roast").required(true)),

        CreateCommand::new("compliment")
            .description("Compliment a user")
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to compliment").required(true)),

        CreateCommand::new("coinflip")
            .description("Flip a coin"),

        CreateCommand::new("rps")
            .description("Rock, Paper, Scissors")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "choice", "rock, paper, or scissors")
                    .required(true)
                    .add_string_choice("Rock", "rock")
                    .add_string_choice("Paper", "paper")
                    .add_string_choice("Scissors", "scissors"),
            ),
    ]
}

pub async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    match interaction.data.name.as_str() {
        "8ball"      => eightball(ctx, interaction).await,
        "dice"       => dice(ctx, interaction).await,
        "roast"      => roast(ctx, interaction).await,
        "compliment" => compliment(ctx, interaction).await,
        "coinflip"   => coinflip(ctx, interaction).await,
        "rps"        => rps(ctx, interaction).await,
        _            => Ok(()),
    }
}

async fn eightball(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let answers = [
        "✅ It is certain.", "✅ Without a doubt.", "✅ Yes, definitely.",
        "✅ You may rely on it.", "✅ As I see it, yes.", "✅ Most likely.",
        "🤔 Reply hazy, try again.", "🤔 Ask again later.", "🤔 Cannot predict now.",
        "❌ Don't count on it.", "❌ Very doubtful.", "❌ My sources say no.",
        "❌ Outlook not so good.", "❌ No.", "❌ My reply is no.",
    ];

    let question = interaction.data.options().iter()
        .find(|o| o.name == "question")
        .and_then(|o| o.value.as_str())
        .unwrap_or("...");

    let answer = answers[rand::thread_rng().gen_range(0..answers.len())];

    let embed = CreateEmbed::new()
        .title("🎱 Magic 8-Ball")
        .color(0x00d2ff)
        .field("Question", question, false)
        .field("Answer", answer, false);

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed)
    )).await
}

async fn dice(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let sides = interaction.data.options().iter()
        .find(|o| o.name == "sides")
        .and_then(|o| o.value.as_i64())
        .unwrap_or(6)
        .max(2) as u64;

    let roll = rand::thread_rng().gen_range(1..=sides);

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("🎲 You rolled a **{}** (d{})", roll, sides))
    )).await
}

async fn roast(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let roasts = [
        "Your code has more bugs than a rainforest.",
        "I've seen better decisions made by a coin flip.",
        "You're the human equivalent of a 404 error.",
        "Your wifi signal is stronger than your arguments.",
        "Even your shadow doesn't want to follow you around.",
    ];

    let user_id = interaction.data.options().iter()
        .find(|o| o.name == "user")
        .and_then(|o| o.value.as_user_id())
        .unwrap_or(interaction.user.id);

    let roast = roasts[rand::thread_rng().gen_range(0..roasts.len())];

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("🔥 <@{}>: {}", user_id, roast))
    )).await
}

async fn compliment(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let compliments = [
        "You're a ray of sunshine in a world full of clouds! ☀️",
        "Your potential is limitless. Keep shining! ⭐",
        "The world is a better place with you in it! 🌍",
        "You make everything look effortless! 💪",
        "You have the best energy in any room! ✨",
    ];

    let user_id = interaction.data.options().iter()
        .find(|o| o.name == "user")
        .and_then(|o| o.value.as_user_id())
        .unwrap_or(interaction.user.id);

    let comp = compliments[rand::thread_rng().gen_range(0..compliments.len())];

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("💙 <@{}>: {}", user_id, comp))
    )).await
}

async fn coinflip(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let result = if rand::thread_rng().gen_bool(0.5) { "🪙 Heads!" } else { "🪙 Tails!" };

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(result)
    )).await
}

async fn rps(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let choices = ["rock", "paper", "scissors"];
    let emojis = [("rock", "🪨"), ("paper", "📄"), ("scissors", "✂️")];

    let player = interaction.data.options().iter()
        .find(|o| o.name == "choice")
        .and_then(|o| o.value.as_str())
        .unwrap_or("rock");

    let bot = choices[rand::thread_rng().gen_range(0..3)];

    let player_emoji = emojis.iter().find(|(k, _)| *k == player).map(|(_, v)| v).unwrap_or(&"❓");
    let bot_emoji = emojis.iter().find(|(k, _)| *k == bot).map(|(_, v)| v).unwrap_or(&"❓");

    let result = match (player, bot) {
        (p, b) if p == b => "🤝 It's a tie!",
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "🎉 You win!",
        _ => "😈 I win!",
    };

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!(
                "You: {} {} vs Bot: {} {}\n\n**{}**",
                player_emoji, player, bot_emoji, bot, result
            ))
    )).await
      }
      
