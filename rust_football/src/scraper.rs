use scraper::{Html, Selector};
use crate::models::{PlayerCA, PlayerPass};

pub fn get_url(url: &str) -> Html {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .unwrap();
    let html = response.text().unwrap();
    
    let uncommented_html = html.replace("<!--", "").replace("-->", "");
    Html::parse_document(&uncommented_html)
}

pub fn extract_ca_stats(document: Html, table_selector: &str) -> Vec<PlayerCA> {
    let mut players = Vec::new();
    let table_selector = Selector::parse(table_selector).unwrap();

    if let Some(table) = document.select(&table_selector).next() {
        let row_selector = Selector::parse("tbody tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        for row in table.select(&row_selector) {
            let cells: Vec<String> = row.select(&cell_selector)
                .map(|cell| cell.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .collect();

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
    players
}

pub fn extract_pass_stats(document: Html, table_selector: &str) -> Vec<PlayerPass> {
    let mut players = Vec::new();
    let table_selector = Selector::parse(table_selector).unwrap();

    if let Some(table) = document.select(&table_selector).next() {
        let row_selector = Selector::parse("tbody tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        for row in table.select(&row_selector) {
            let cells: Vec<String> = row.select(&cell_selector)
                .map(|cell| cell.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .collect();

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
    players
}