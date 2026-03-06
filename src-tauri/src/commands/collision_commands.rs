use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
use tokio::sync::RwLock;
use std::sync::Arc;
use tauri::{command, Manager, State};

/// 已撞卡号存储
pub struct CollisionStore {
    used_cards: Arc<RwLock<HashSet<String>>>,
    store_path: PathBuf,
}

impl CollisionStore {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let app_data_dir = app_handle.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;
        
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
        
        let store_path = app_data_dir.join("collision_cards.json");
        let used_cards = Self::load_cards(&store_path)?;
        
        Ok(Self {
            used_cards: Arc::new(RwLock::new(used_cards)),
            store_path,
        })
    }
    
    fn load_cards(path: &PathBuf) -> Result<HashSet<String>, String> {
        if path.exists() {
            let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
            match serde_json::from_str::<Vec<String>>(&data) {
                Ok(cards) => Ok(cards.into_iter().collect()),
                Err(e) => {
                    println!("[CollisionStore] 配置文件格式错误，将重置: {}", e);
                    let _ = fs::remove_file(path);
                    Ok(HashSet::new())
                }
            }
        } else {
            Ok(HashSet::new())
        }
    }
    
    async fn save(&self) -> Result<(), String> {
        let cards = self.used_cards.read().await;
        let cards_vec: Vec<String> = cards.iter().cloned().collect();
        let data = serde_json::to_string_pretty(&cards_vec).map_err(|e| e.to_string())?;
        fs::write(&self.store_path, data).map_err(|e| e.to_string())
    }
    
    pub async fn add_card(&self, card_number: &str) -> Result<(), String> {
        // 移除卡号中的空格，统一存储格式
        let normalized = card_number.replace(" ", "");
        let mut cards = self.used_cards.write().await;
        cards.insert(normalized);
        drop(cards);
        self.save().await
    }
    
    pub async fn add_cards_batch(&self, card_numbers: Vec<String>) -> Result<(), String> {
        let mut cards = self.used_cards.write().await;
        for card in card_numbers {
            let normalized = card.replace(" ", "");
            cards.insert(normalized);
        }
        drop(cards);
        self.save().await
    }
    
    pub async fn contains(&self, card_number: &str) -> bool {
        let normalized = card_number.replace(" ", "");
        let cards = self.used_cards.read().await;
        cards.contains(&normalized)
    }
    
    pub async fn get_all(&self) -> Vec<String> {
        let cards = self.used_cards.read().await;
        cards.iter().cloned().collect()
    }
    
    pub async fn count(&self) -> usize {
        let cards = self.used_cards.read().await;
        cards.len()
    }
    
    pub async fn clear(&self) -> Result<(), String> {
        let mut cards = self.used_cards.write().await;
        cards.clear();
        drop(cards);
        self.save().await
    }
}

/// 添加已撞卡号
#[command]
pub async fn add_collision_card(
    collision_store: State<'_, Arc<CollisionStore>>,
    card_number: String,
) -> Result<(), String> {
    collision_store.add_card(&card_number).await
}

/// 批量添加已撞卡号
#[command]
pub async fn add_collision_cards_batch(
    collision_store: State<'_, Arc<CollisionStore>>,
    card_numbers: Vec<String>,
) -> Result<(), String> {
    collision_store.add_cards_batch(card_numbers).await
}

/// 检查卡号是否已撞
#[command]
pub async fn check_collision_card(
    collision_store: State<'_, Arc<CollisionStore>>,
    card_number: String,
) -> Result<bool, String> {
    Ok(collision_store.contains(&card_number).await)
}

/// 获取所有已撞卡号
#[command]
pub async fn get_all_collision_cards(
    collision_store: State<'_, Arc<CollisionStore>>,
) -> Result<Vec<String>, String> {
    Ok(collision_store.get_all().await)
}

/// 获取已撞卡号数量
#[command]
pub async fn get_collision_cards_count(
    collision_store: State<'_, Arc<CollisionStore>>,
) -> Result<usize, String> {
    Ok(collision_store.count().await)
}

/// 清空已撞卡号
#[command]
pub async fn clear_collision_cards(
    collision_store: State<'_, Arc<CollisionStore>>,
) -> Result<(), String> {
    collision_store.clear().await
}

