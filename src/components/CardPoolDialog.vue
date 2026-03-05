<template>
  <el-dialog
    v-model="visible"
    width="800px"
    :close-on-click-modal="false"
    title="虚拟卡池"
    class="card-pool-dialog"
  >
    <div class="card-pool-content">
      <div class="pool-header">
        <div class="header-info">
          <el-icon><Collection /></el-icon>
          <span>虚拟卡池管理</span>
          <el-tag size="small" type="info" class="count-tag">{{ cardsStore.cards.length }}</el-tag>
        </div>
        <el-input 
          style="width: 300px;" 
          v-model="cardNumber" 
          placeholder="添加卡号（16位数字）"
          @keyup.enter="addCardToPool"
        />
        <el-button type="primary" :icon="Plus" @click="addCardToPool" :loading="cardsStore.loading">添加卡片</el-button>
      </div>

      <div class="pool-list">
        <el-table :data="cardsStore.cards" stripe style="width: 100%" max-height="500" v-loading="cardsStore.loading">
          <el-table-column prop="card_number" label="卡号" width="400">
            <template #default="{ row }">
              <span class="card-number">{{ formatCardNumber(row.card_number) }}</span>
            </template>
          </el-table-column>
        
          <el-table-column prop="last_success_time" label="最近成功时间" width="180">
            <template #default="{ row }">
              <span v-if="row.last_success_time">{{ row.last_success_time }}</span>
              <span v-else style="color: #999;">-</span>
            </template>
          </el-table-column>
          <el-table-column prop="enabled" label="是否启用" width="120">
            <template #default="{ row }">
              <el-switch
                v-model="row.enabled"
                :active-value="true"
                :inactive-value="false"
                @change="(val: boolean) => toggleEnabled(row.id, val)"
                :loading="cardsStore.loading"
              />
            </template>
          </el-table-column>
          <el-table-column label="操作" width="100" fixed="right">
            <template #default="{ row }">
              <el-button 
                type="danger" 
                :icon="Delete" 
                size="small" 
                @click="removeCard(row.id)"
                :loading="cardsStore.loading"
              >
                删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>

        <div v-if="cardsStore.cards.length === 0 && !cardsStore.loading" class="empty-state">
          <el-empty description="卡池为空，请添加卡片" />
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="visible = false">关闭</el-button>
      
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { Plus, Delete, Collection } from '@element-plus/icons-vue';
import { useCardsStore } from '@/store/modules/cards';

const cardNumber = ref('');

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
});

const cardsStore = useCardsStore();

// 监听弹框打开，加载卡片数据
watch(visible, (newVal) => {
  if (newVal) {
    cardsStore.loadCards();
  }
});

// 格式化卡号显示（每4位加空格）
function formatCardNumber(cardNumber: string): string {
  return cardsStore.formatCardNumber(cardNumber);
}

// 添加卡片到卡池
async function addCardToPool() {
  if (!cardNumber.value.trim()) {
    ElMessage.warning('请输入卡号');
    return;
  }

  try {
    await cardsStore.addCardByNumber(cardNumber.value);
    cardNumber.value = ''; // 清空输入框
  } catch (error) {
    // 错误已在 store 中处理
  }
}

// 移除卡片
async function removeCard(id: string) {
  try {
    await cardsStore.deleteCard(id);
  } catch (error) {
    // 错误已在 store 中处理
  }
}

// 切换启用状态
async function toggleEnabled(id: string, enabled: boolean) {
  try {
    await cardsStore.updateCardEnabled(id, enabled);
  } catch (error) {
    // 失败时恢复之前状态（重新加载列表以保证一致性）
    cardsStore.loadCards();
  }
}


</script>

<style scoped>
.card-pool-content {
  padding: 0;
}

.pool-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.header-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.header-info .el-icon {
  font-size: 20px;
  color: var(--el-color-primary);
}

.count-tag {
  margin-left: 8px;
}

.pool-list {
  min-height: 300px;
}

.card-number {
  font-family: 'Courier New', monospace;
  font-weight: 500;
  letter-spacing: 1px;
}

.empty-state {
  padding: 40px 0;
  text-align: center;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>

