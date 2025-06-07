<template>
    <div class="content">
        <div class="list-container">
            <p class="user-select-none">Zainstalowane</p>
            <InstalledItem v-for="item in installed" :item=item />
            <p v-if="installed.length === 0" class="list-placeholder">Pusto</p>
        </div>
        <div class="list-container">
            <p class="user-select-none">Dostępne</p>
            <AvailableItem v-for="item in available" :item=item />
            <p v-if="available.length === 0" class="list-placeholder">Pusto</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import AvailableItem from './AvailableItem.vue';
import InstalledItem from './InstalledItem.vue';
import { ModFile } from '../../types/ModFile';

const props = defineProps({
    installed: {
        type: Object as () => ModFile[],
        required: true
    },
    available: {
        type: Object as () => ModFile[],
        required: true
    }
})
</script>

<style scoped>
.content {
  width: 100%;
  display: flex;
  flex-direction: row;
  align-items: flex-start;
  justify-content: space-around;
  gap: 20px;
}

.list-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}


.list-placeholder {
  color: #777;
  user-select: none;
}


</style>