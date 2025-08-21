use std::str;
use scraper::{Html, Selector};

#[derive(Debug)]
struct PlayerCA {
    player: String,
    ninety_mins: String,
    gca90: String,
    sca90: String
}

#[derive(Debug)]
struct PlayerPass {
    player: String,
    ninety_mins: String,
    exp_assist: String,
    key_passes: String,
    penalty_area_passes: String,
    penalty_area_crosses: String
}

impl PlayerPass {
    // These functions turn our strings into f64 using parse
    fn ninety_mins(&self) -> f64 {
        self.ninety_mins.parse().unwrap()
    }
    
    fn x_a(&self) -> f64 {
        self.exp_assist.parse().unwrap()
    }
    
    fn key_passes(&self) -> f64 {
        self.key_passes.parse().unwrap()
    }

    fn penalty_area_passes(&self) -> f64 {
        self.penalty_area_passes.parse().unwrap()
    }

    fn penalty_area_crosses(&self) -> f64 {
        self.penalty_area_crosses.parse().unwrap()
    }

    fn is_regular_starter(&self) -> bool {
        self.ninety_mins() > 0.7
    }

    fn pass_danger(&self) -> f64 {
        // Weighted combination of passing threat metrics
        (self.x_a() * 2.0) +                    // xA is most important (goals expected from assists)
        (self.key_passes() * 1.5) +             // Key passes create chances
        (self.penalty_area_passes() * 1.2) +    // Passes into dangerous areas
        (self.penalty_area_crosses() * 1.0)     // Crosses into the box
    }

    fn pass_danger_per_90(&self) -> f64 {
        let total_danger = self.pass_danger();
        if self.ninety_mins() > 0.0 {
            total_danger / self.ninety_mins()
        } else {
            0.0
        }
    }

}

impl PlayerCA {
    // These functions turn our strings into f64 using parse
    fn ninety_mins(&self) -> f64 {
        self.ninety_mins.parse().unwrap()
    }
    
    fn sca90(&self) -> f64 {
        self.sca90.parse().unwrap()
    }
    
    fn gca90(&self) -> f64 {
        self.gca90.parse().unwrap()
    }
    
    fn is_regular_starter(&self) -> bool {
        self.ninety_mins() > 0.7
    }
    
    fn creativity_score(&self) -> f64 {
        (self.sca90()) + (self.gca90())
    }
}

#[derive(Debug)]
struct PlayerRecommendation {
    player: String,
    overall_score: f64,
    creativity: f64,
    passing_threat: f64,
}

impl PlayerRecommendation {
    fn new(ca_player: &PlayerCA, pass_player: &PlayerPass) -> Option<Self> {
        // Only recommend if names match and both players have meaningful stats
        if ca_player.player == pass_player.player && 
           ca_player.is_regular_starter() && 
           pass_player.is_regular_starter() {
            
            let creativity = ca_player.creativity_score();
            let passing_threat = pass_player.pass_danger_per_90();
            
            // Only recommend if they have actual stats (not zeros from failed parsing)
            if creativity > 0.0 && passing_threat > 0.0 {
                let overall_score = (creativity * 0.6) + (passing_threat * 0.4);
                
                Some(PlayerRecommendation {
                    player: ca_player.player.clone(),
                    overall_score,
                    creativity,
                    passing_threat,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn main() {
    // ====================== SHOT CREATING ACTIONS AND GOAL CREATING ACTIONS ======================

    // Specify what URL we want to extract from, then get all the HTML
    let url = "https://fbref.com/en/comps/9/gca/Premier-League-Stats";
    let document = get_url(url);

    // Select what table ID we want, then get all the players from that table
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
    // xA, Key Passes, Passes into Penalty Box, Crosses into Penalty Box

    // Specify what URL we want to extract from, then get all the HTML
    let url = "https://fbref.com/en/comps/9/passing/Premier-League-Stats";
    let document = get_url(url);

    // Select what table ID we want, then get all the players from that table
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

fn get_url(url: &str) -> Html {
    // We need to actually specify the following so that fbref.com doesn't think we are a bot
    let client = reqwest::blocking::Client::new(); // Reusable HTTP client
    let response = client
        .get(url) // Make the GET request to the URL
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36") // Adds browser headers to look like a real browser
        .send() // Send the request
        .unwrap(); // Panics for us if the request fails, but also we need to unwrap anyway
    let html = response.text().unwrap();

    // Seems this comment part actually stops us from parsing the HTML as easily! 
    let uncommented_html = html.replace("<!--", "").replace("-->", "");

    // And then we can parse to find the table we want, which has an ID of 'stats_gca'
    let document = Html::parse_document(&uncommented_html);
    return document
}

fn extract_ca_stats(document: Html, table_selector: &str) -> Vec<PlayerCA> {
    // Lets now create our players vector, containing the data we care about
    let mut players = Vec::new();

    let table_selector = Selector::parse(table_selector).unwrap();

    // Find the table and parse its data if it exists
    if let Some(table) = document.select(&table_selector).next() {

        // Create selectors for table rows and cells
        let row_selector = Selector::parse("tbody tr").unwrap(); // Select all rows in table body
        let cell_selector = Selector::parse("td").unwrap(); // Select all data cells

        // Iterate through each row in the table
        for row in table.select(&row_selector) {

            // Extract text from each cell in the row and clean it up
            let cells: Vec<String> = row.select(&cell_selector)
                .map(|cell| cell.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .collect();

            // And here we just push the relevant stats that we want to the player vector
            if cells.len() > 5 {
                let player = PlayerCA {
                    player: cells[0].clone(),
                    ninety_mins: cells[6].clone(),
                    gca90: cells[16].clone(),
                    sca90: cells[8].clone(),
                };
                players.push(player);
            }
        }
    }
    return players
}

fn top_3_sca(players: &Vec<PlayerCA>) -> Vec<&PlayerCA> { // Need to return a reference, .iter() creates Player references
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect(); // Collect all the filtered players from this iter
    
    // Here we order the vector by sca90
    filtered_players.sort_by(|a, b| {
        let sca_a = a.sca90();
        let sca_b = b.sca90();
        sca_b.partial_cmp(&sca_a).unwrap()
    });
    return filtered_players
}

fn top_3_gca(players: &Vec<PlayerCA>) -> Vec<&PlayerCA> { // Need to return a reference, .iter() creates Player references
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect(); // Collect all the filtered players from this iter
    
    // Here we order the vector by gca90
    filtered_players.sort_by(|a, b| {
        let gca_a = a.gca90();
        let gca_b = b.gca90();
        gca_b.partial_cmp(&gca_a).unwrap()
    });
    return filtered_players
}

fn top_3_threat(players: &Vec<PlayerCA>) -> Vec<&PlayerCA> { // Need to return a reference, .iter() creates Player references
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect(); // Collect all the filtered players from this iter
    
    filtered_players.sort_by(|a, b| {
        let threat_a = a.creativity_score();
        let threat_b = b.creativity_score();
        threat_b.partial_cmp(&threat_a).unwrap()
    });
    return filtered_players
}

fn extract_pass_stats(document: Html, table_selector: &str) -> Vec<PlayerPass> {
    // Lets now create our players vector, containing the data we care about
    let mut players = Vec::new();

    let table_selector = Selector::parse(table_selector).unwrap();

    // Find the table and parse its data if it exists
    if let Some(table) = document.select(&table_selector).next() {

        // Create selectors for table rows and cells
        let row_selector = Selector::parse("tbody tr").unwrap(); // Select all rows in table body
        let cell_selector = Selector::parse("td").unwrap(); // Select all data cells

        // Iterate through each row in the table
        for row in table.select(&row_selector) {

            // Extract text from each cell in the row and clean it up
            let cells: Vec<String> = row.select(&cell_selector)
                .map(|cell| cell.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .collect();

            // And here we just push the relevant stats that we want to the player vector
            if cells.len() > 5 {
                let player = PlayerPass {
                    player: cells[0].clone(),
                    ninety_mins: cells[6].clone(),
                    exp_assist: cells[23].clone(),
                    key_passes: cells[25].clone(),
                    penalty_area_passes: cells[27].clone(),
                    penalty_area_crosses: cells[28].clone(),
                };
                players.push(player);
            }
        }
    }
    return players
}

fn top_3_xa(players: &Vec<PlayerPass>) -> Vec<&PlayerPass> { // Need to return a reference, .iter() creates Player references
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect(); // Collect all the filtered players from this iter
    
    // Here we order the vector by sca90
    filtered_players.sort_by(|a, b| {
        let x_a_a = a.x_a();
        let x_a_b = b.x_a();
        x_a_b.partial_cmp(&x_a_a).unwrap()
    });
    return filtered_players
}

fn top_3_pass_danger(players: &Vec<PlayerPass>) -> Vec<&PlayerPass> { // Need to return a reference, .iter() creates Player references
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect(); // Collect all the filtered players from this iter
    
    filtered_players.sort_by(|a, b| {
        let pass_danger_a = a.pass_danger_per_90();
        let pass_danger_b = b.pass_danger_per_90();
        pass_danger_b.partial_cmp(&pass_danger_a).unwrap()
    });
    return filtered_players
}

fn weekly_recommendation(ca_players: &[PlayerCA], pass_players: &[PlayerPass]) -> Option<PlayerRecommendation> {
    let mut recommendations = Vec::new();
    let mut matches_found = 0;
    
    // Match players across datasets
    for ca_player in ca_players {
        if let Some(pass_player) = pass_players.iter().find(|p| p.player == ca_player.player) {
            matches_found += 1;
            if let Some(rec) = PlayerRecommendation::new(ca_player, pass_player) {
                recommendations.push(rec);
            }
        }
    }
    // Sort by overall score and return best
    recommendations.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());
    recommendations.into_iter().next()
}