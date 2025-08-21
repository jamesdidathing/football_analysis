mod models;
mod scraper;
mod analysis;

use scraper::{get_url, extract_ca_stats, extract_pass_stats};
use analysis::{top_3_sca, top_3_gca, top_3_threat, top_3_xa, top_3_pass_danger, weekly_recommendation};

fn main() {
    // ====================== SHOT CREATING ACTIONS AND GOAL CREATING ACTIONS ======================

    let url = "https://fbref.com/en/comps/9/gca/Premier-League-Stats";
    let document = get_url(url);
    let table_selector = "table#stats_gca";
    let ca_players = extract_ca_stats(document, table_selector);

    println!("====================== SHOT CREATING ACTIONS AND GOAL CREATING ACTIONS ======================");

    let filtered_players: Vec<_> = top_3_sca(&ca_players);
    println!("Top 3 SCA90:");
    for (i, player) in filtered_players.iter().take(3).enumerate() { 
        println!("{}. {} - SCA90: {}", i+1, player.player, player.sca90);
    }
    
    let filtered_players: Vec<_> = top_3_gca(&ca_players);
    println!("\nTop 3 GCA90:");
    for (i, player) in filtered_players.iter().take(3).enumerate() {
        println!("{}. {} - GCA90: {}", i+1, player.player, player.gca90);
    }

    let filtered_players: Vec<_> = top_3_threat(&ca_players);
    println!("\nTop 3 GCA90 + SCA90:");
    for (i, player) in filtered_players.iter().take(3).enumerate() {
        println!("{}. {} - Creativity Score: {}", i+1, player.player, player.creativity_score());
    }

    // ====================== PASSING ======================

    let url = "https://fbref.com/en/comps/9/passing/Premier-League-Stats";
    let document = get_url(url);
    let table_selector = "table#stats_passing";
    let pass_players = extract_pass_stats(document, table_selector);

    println!("====================== EXPECTED ASSISTS AND PASS THREAT ======================");

    let filtered_players: Vec<_> = top_3_xa(&pass_players);
    println!("\nTop 3 Expected Assists:");
    for (i, player) in filtered_players.iter().take(3).enumerate() {
        println!("{}. {} - xA: {}", i+1, player.player, player.x_a());
    }

    let filtered_players: Vec<_> = top_3_pass_danger(&pass_players);
    println!("\nTop 3 Pass Threats:");
    for (i, player) in filtered_players.iter().take(3).enumerate() {
        println!("{}. {} - Pass Threat per 90: {:.2}", i+1, player.player, player.pass_danger_per_90());
    }

    // ====================== WEEKLY RECOMMENDATION ======================
    
    if let Some(recommendation) = weekly_recommendation(&ca_players, &pass_players) {
        println!("\n====================== WEEKLY RECOMMENDATION ======================");
        println!("Recommended Player: {}", recommendation.player);
        println!("Overall Score: {:.2}", recommendation.overall_score);
        println!("Creativity: {:.2}", recommendation.creativity);
        println!("Passing Threat: {:.2}", recommendation.passing_threat);
    }
}