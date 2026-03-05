import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { cardApi } from '@/api';
import { ElMessage } from 'element-plus';
import type { VirtualCard } from '@/utils/cardGenerator';

export interface PoolCard {
  id: string;
  card_number: string;
  expiry_date: string;
  cvv: string;
  cardholder_name: string;
  billing_address: {
    street_address: string;
    street_address_line2: string;
    city: string;
    district?: string;
    state: string;
    postal_code: string;
    country: string;
  };
  last_success_time?: string;
  enabled?: boolean;
}

export const useCardsStore = defineStore('cards', () => {
  const cards = ref<PoolCard[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 加载所有卡片
  async function loadCards() {
    loading.value = true;
    error.value = null;
    try {
      cards.value = await cardApi.getAllCards();
    } catch (e) {
      error.value = (e as Error).message;
      ElMessage.error(`加载卡池失败: ${error.value}`);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 添加卡片到卡池（生成新卡）
  async function addCardToPool() {
    loading.value = true;
    error.value = null;
    try {
      const newCard = await cardApi.addCardToPool();
      cards.value.push(newCard);
      ElMessage.success('卡片已添加到卡池');
      return newCard;
    } catch (e) {
      error.value = (e as Error).message;
      ElMessage.error(`添加卡片失败: ${error.value}`);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 通过卡号添加卡片
  async function addCardByNumber(cardNumber: string) {
    loading.value = true;
    error.value = null;
    try {
      // 移除空格
      const cleanNumber = cardNumber.replace(/\s/g, '');
      
      // 验证卡号
      if (cleanNumber.length !== 16) {
        throw new Error('卡号必须是16位数字');
      }
      
      if (!/^\d+$/.test(cleanNumber)) {
        throw new Error('卡号只能包含数字');
      }

      const newCard = await cardApi.addCardByNumber(cleanNumber);
      cards.value.push(newCard);
      ElMessage.success('卡片已添加到卡池');
      return newCard;
    } catch (e) {
      error.value = (e as Error).message;
      ElMessage.error(`添加卡片失败: ${error.value}`);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 删除卡片
  async function deleteCard(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await cardApi.deleteCard(id);
      cards.value = cards.value.filter(c => c.id !== id);
      ElMessage.success('卡片已删除');
    } catch (e) {
      error.value = (e as Error).message;
      ElMessage.error(`删除卡片失败: ${error.value}`);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 更新卡片的最近成功时间
  async function updateCardSuccessTime(id: string, successTime: string) {
    error.value = null;
    try {
      await cardApi.updateCardSuccessTime(id, successTime);
      const card = cards.value.find(c => c.id === id);
      if (card) {
        card.last_success_time = successTime;
      }
    } catch (e) {
      error.value = (e as Error).message;
      ElMessage.error(`更新成功时间失败: ${error.value}`);
      throw e;
    }
  }

  // 更新卡片启用状态
  async function updateCardEnabled(id: string, enabled: boolean) {
    error.value = null;
    try {
      await cardApi.updateCardEnabled(id, enabled);
      const card = cards.value.find(c => c.id === id);
      if (card) {
        card.enabled = enabled;
      }
    } catch (e) {
      error.value = (e as Error).message;
      ElMessage.error(`更新启用状态失败: ${error.value}`);
      throw e;
    }
  }

  // 格式化卡号显示（每4位加空格）
  function formatCardNumber(cardNumber: string): string {
    const clean = cardNumber.replace(/\s/g, '');
    return clean.replace(/(\d{4})(?=\d)/g, '$1 ');
  }

  return {
    // State
    cards,
    loading,
    error,
    
    // Actions
    loadCards,
    addCardToPool,
    addCardByNumber,
    deleteCard,
    updateCardSuccessTime,
    updateCardEnabled,
    
    // Helpers
    formatCardNumber,
  };
});

