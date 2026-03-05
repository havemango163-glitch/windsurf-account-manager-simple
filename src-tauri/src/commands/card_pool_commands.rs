use crate::models::PoolCard;
use crate::repository::DataStore;
use crate::utils::card_generator::{CardGenerator, VirtualCard};
use std::sync::Arc;
use tauri::State;

/// 添加卡片到卡池（通过生成虚拟卡）
#[tauri::command]
pub async fn add_card_to_pool(
    store: State<'_, Arc<DataStore>>,
) -> Result<PoolCard, String> {
    // 生成虚拟卡
    let card = CardGenerator::generate_card();
    
    // 添加到卡池
    store.add_card_to_pool(card)
        .await
        .map_err(|e| e.to_string())
}

/// 通过卡号添加卡片到卡池
#[tauri::command]
pub async fn add_card_by_number(
    card_number: String,
    store: State<'_, Arc<DataStore>>,
) -> Result<PoolCard, String> {
    store.add_card_by_number(card_number)
        .await
        .map_err(|e| e.to_string())
}

/// 获取所有卡池中的卡片
#[tauri::command]
pub async fn get_all_cards(
    store: State<'_, Arc<DataStore>>,
) -> Result<Vec<PoolCard>, String> {
    store.get_all_cards()
        .await
        .map_err(|e| e.to_string())
}

/// 删除卡池中的卡片
#[tauri::command]
pub async fn delete_card_from_pool(
    id: String,
    store: State<'_, Arc<DataStore>>,
) -> Result<(), String> {
    store.delete_card_from_pool(id)
        .await
        .map_err(|e| e.to_string())
}

/// 更新卡片的最近成功时间
#[tauri::command]
pub async fn update_card_success_time(
    id: String,
    success_time: String,
    store: State<'_, Arc<DataStore>>,
) -> Result<(), String> {
    store.update_card_success_time(id, success_time)
        .await
        .map_err(|e| e.to_string())
}

/// 更新卡片的启用状态
#[tauri::command]
pub async fn update_card_enabled(
    id: String,
    enabled: bool,
    store: State<'_, Arc<DataStore>>,
) -> Result<(), String> {
    store.update_card_enabled(id, enabled)
        .await
        .map_err(|e| e.to_string())
}

