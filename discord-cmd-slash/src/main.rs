
use jamclub_seaorm::entities::sea_orm_active_enums::*;
use jamclub_seaorm::entities::locations;
use jamclub_seaorm::entities::encounters;
use jamclub_seaorm::entities::members;

use jamclub_seaorm::model_controller::{EventManager, EncounterManager, MemberManager};
use jamclub_seaorm::entities::display::*;
use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
use poise::CreateReply;
use sea_orm::{Database, DatabaseConnection};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::all::Timestamp;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    println!("On y va !");
    log::info!("initialize bot");

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![activites2(), lien_formulaire2()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    log::info!("starting bot");
    client.unwrap().start().await.unwrap();
}

#[derive(poise::ChoiceParameter)]
enum Formulaire {
    #[name = "Ajouter une rencontre"]
    AjouterRencontre,
    #[name = "Copier une rencontre et la modifier"]
    CopierRencontre
}

async fn get_head_embed() -> CreateEmbed {
    log::info!("head embed");
    CreateEmbed::new()
        .title("Asso Zikapanam\nLa musique, ensemble !")
        .image("https://www.blog.zikapanam.fr/assets/images/E68A4812-Paul_CUVEILLIER-EDIT.jpg")
        .footer(CreateEmbedFooter::new("Crédit Photo: Paul CUVEILLIER"))
        .timestamp(Timestamp::now())
}

async fn get_links_embed() -> CreateEmbed  {
    log::info!("links embed");
    CreateEmbed::new()
        .title("Créer une rencontre (pour les cotisants)")
        .description("**DOC : [Créer/Editer/Annuler/Réaliser une rencontre](https://docs.google.com/document/d/1FKXJUnDX1S1y4mT7jh6S6Cvvz__qZNgOmbK1GcPJjQE/edit#heading=h.f2g60lx3k85e)**\n**Liens ;**\n- [Ajouter une rencontre](https://forms.fillout.com/t/6fceGy7qxvus)\n- [Copier et modifier une rencontre](https://form.fillout.com/t/qNJnjGZWPFus)")
        .color(1240292)
        .timestamp(Timestamp::now())
}

async fn get_help_embed() -> CreateEmbed {
    log::info!("help embed");
    CreateEmbed::new()
        .title("Guide du serveur (Aide)")
        .description("Clique sur ce lien  <id:guide> et consulte la section qui t'intéresse")
        .timestamp(Timestamp::now())
}

fn get_encounter_libelle(encounter : encounters::Model, location : Option<locations::Model>, community : Community) -> String {

    let enc_date = encounter.start_date;
    let dateformatted = enc_date.format("%A %v %H:%M"); 
    if let Some(loc) = location {
        format!("{} | {} | {} | {} | {} | {} | {}", encounter.id, DisplayableVec(encounter.communities), encounter.long_title, encounter.encounter_type, dateformatted, loc.long_title, loc.postal_code)
    } else {
        format!("{} | {} | {} | {} | {} | Lieu en description", encounter.id, community, encounter.long_title, encounter.encounter_type, dateformatted)
    }
}

async fn get_encounters_embed(limit : u64, community : Community, db : &DatabaseConnection ) -> CreateEmbed {
    log::info!("encounters embed");
    let mut message = String::from("");
    let mut num = 1;
    let encounters = EncounterManager::find_encounters_to_come_and_to_fill_in_by_communities(vec![community.clone()], limit, &db).await;    
    match encounters {
        Ok(encs) => {
            for encounter in encs {
                let location = EncounterManager::get_location(encounter.clone(), db).await;
                match location {
                    Ok(loc) => {
                        let libelle = get_encounter_libelle(encounter.clone(), loc, community.clone());
                        if let Some(url) = encounter.discord_url_message {
                            let addmsg = format!("\n**n°{}** : [{}]({}) \n", num, libelle, url);
                            message.push_str(&addmsg); 
                            num += 1;
                        }        
                    },
                    Err(e) => {eprintln!("Error retrieving location {}", e);},
                }
            }
        },
        Err(e) => {eprintln!("Error retrieving encounters {}", e);},
    }
    message.push_str("\n\n**Toutes les rencontres ouvertes sont là : <#1079705665016188971>**");

    CreateEmbed::new()
            .title(format!("Rencontres ouvertes pour les {}", community.clone()))
            .color(16257544)
            .description(message)
            .timestamp(Timestamp::now())
}

async fn get_events_embed(limit : u64, db : &DatabaseConnection ) -> CreateEmbed {
    log::info!("events embed");
    let mut message = String::from("");
    let events = EventManager::find_events_to_come(limit, &db).await;

    match events {
        Ok(evts) => {
            let mut num = 1;
            for event in evts {
                if let Some(mobilizon_url) = event.mobilizon_url {
                    let addmsg = format!("\n**n°{}** : [{}]({})", num, event.title, mobilizon_url);
                    message.push_str(&addmsg); 
                
                    num += 1;
                }
            }
        },
        Err(err) => {eprintln!("Error processing events: {:?}", err); },
    }
    
    message.push_str("\n\n**[Tous les événements publiques à venir sont sur Mobilizon](<https://keskonfai.fr/@asso_zikapanam/events>)**");

    CreateEmbed::new()
            .title("Les événements publics")
            .color(644393)
            .description(message)
            .timestamp(Timestamp::now())
}

async fn get_enrollment_embed(member : members::Model) -> CreateEmbed {
    log::info!("enrollment embed");

    let mut message = String::from("");
    if member.communities.contains(&Community::Interbeg) {
        let enrollment = "**Interdebs :**\n- Collectifs : <#1048716129117409320>\n- <#1048706149471498350>\n\n";
        message.push_str(&enrollment);
    }
    if member.communities.contains(&Community::Interadv) {
        let enrollment = "**Avancés :**\n- Collectifs : <#1220509942746972342>\n- <#1220511824647032902>\n\n";
        message.push_str(&enrollment);
    }

    CreateEmbed::new()
    .title("Recrutements")
    .color(16763480)
    .description(message)
    .timestamp(Timestamp::now())
}

#[poise::command(slash_command)]
async fn activites2(ctx: Context<'_>) -> Result<(), Error> {
    log::info!("/activites called");

    let nickname = ctx.author_member().await.unwrap().display_name().to_string();

    log::debug!("nickname: {}", nickname);

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let db = Database::connect(database_url.clone()).await?;

    let mut discord_reply = CreateReply::default()
                            .content("Consultez ci-dessous les les événements publics (3 max) à venir pour {nickname}")
                            .ephemeral(true);

    discord_reply = discord_reply.embed(get_head_embed().await);
    let themember = MemberManager::find_by_nickname(nickname.clone(), &db).await?;
    if let Some(member) = themember { 
        if member.communities.contains(&Community::Interbeg) {
            discord_reply = discord_reply.embed(get_encounters_embed(3, Community::Interbeg, &db).await);
        }

        if member.communities.contains(&Community::Interadv) {
            discord_reply = discord_reply.embed(get_encounters_embed(3, Community::Interadv, &db).await);
        }
 
        discord_reply = discord_reply.embed(get_enrollment_embed(member).await);
    }

    discord_reply = discord_reply.embed(get_events_embed(3, &db).await);
    discord_reply = discord_reply.embed(get_links_embed().await);
    discord_reply = discord_reply.embed(get_help_embed().await);

    log::info!("writing result to discord");

    let _ = ctx.send(discord_reply).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn lien_formulaire2(ctx: Context<'_>, #[description = "type de formulaire"] formulaire: Formulaire) -> Result<(), Error> {
    let message = "** DOC [Comment créer/modifier/annuler/réaliser une rencontre](<https://docs.google.com/document/d/1FKXUnDX1S1y4mT7jh6S6Cvvz__qZNgOmbK1GcPJjQE/edit#heading=h.f2g60lx3k85e>)**";

    match formulaire {
        Formulaire::AjouterRencontre => {
            let url = "https://forms.fillout.com/t/6fceGy7qxvus";
            let nom_formulaire = "Ajouter une rencontre";
            let content = format!("{}\n\n➡️ Cliquez sur ce lien pour ouvrir [{}]({}).", message, nom_formulaire, url);
            ctx.say(content).await?;
        }

        Formulaire::CopierRencontre => {
            let url = "https://form.fillout.com/t/qNJnjGZWPFus";
            let nom_formulaire = "Copier une rencontre et la modifier";
            let content = format!("{}\n\n➡️ Cliquez sur ce lien pour ouvrir [{}]({}).", message, nom_formulaire, url);
            ctx.say(content).await?;
        }            
    }

    Ok(())
}
