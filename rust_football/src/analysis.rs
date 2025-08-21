use crate::models::{PlayerCA, PlayerPass, PlayerRecommendation};

pub fn top_3_sca(players: &[PlayerCA]) -> Vec<&PlayerCA> {
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect();
    
    filtered_players.sort_by(|a, b| {
        let sca_a = a.sca90();
        let sca_b = b.sca90();
        sca_b.partial_cmp(&sca_a).unwrap()
    });
    filtered_players
}

pub fn top_3_gca(players: &[PlayerCA]) -> Vec<&PlayerCA> {
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect();
    
    filtered_players.sort_by(|a, b| {
        let gca_a = a.gca90();
        let gca_b = b.gca90();
        gca_b.partial_cmp(&gca_a).unwrap()
    });
    filtered_players
}

pub fn top_3_threat(players: &[PlayerCA]) -> Vec<&PlayerCA> {
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect();
    
    filtered_players.sort_by(|a, b| {
        let threat_a = a.creativity_score();
        let threat_b = b.creativity_score();
        threat_b.partial_cmp(&threat_a).unwrap()
    });
    filtered_players
}

pub fn top_3_xa(players: &[PlayerPass]) -> Vec<&PlayerPass> {
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect();
    
    filtered_players.sort_by(|a, b| {
        let xa_a = a.x_a();
        let xa_b = b.x_a();
        xa_b.partial_cmp(&xa_a).unwrap()
    });
    filtered_players
}

pub fn top_3_pass_danger(players: &[PlayerPass]) -> Vec<&PlayerPass> {
    let mut filtered_players: Vec<_> = players.iter()
        .filter(|p| p.is_regular_starter())
        .collect();
    
    filtered_players.sort_by(|a, b| {
        let pass_danger_a = a.pass_danger_per_90();
        let pass_danger_b = b.pass_danger_per_90();
        pass_danger_b.partial_cmp(&pass_danger_a).unwrap()
    });
    filtered_players
}

pub fn weekly_recommendation(ca_players: &[PlayerCA], pass_players: &[PlayerPass]) -> Option<PlayerRecommendation> {
    let mut recommendations = Vec::new();
    let mut matches_found = 0;
    
    for ca_player in ca_players {
        if let Some(pass_player) = pass_players.iter().find(|p| p.player == ca_player.player) {
            matches_found += 1;
            if let Some(rec) = PlayerRecommendation::new(ca_player, pass_player) {
                recommendations.push(rec);
            }
        }
    }
    
    println!("Debug: Found {} name matches, {} valid recommendations", matches_found, recommendations.len());
    
    if recommendations.is_empty() {
        println!("Debug: No valid recommendations found");
        return None;
    }
    
    recommendations.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());
    recommendations.into_iter().next()
}