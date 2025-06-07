<template>
    <div v-if="is_loading" class="loading-container">
        <div class="loading-circle"></div>
        <div class="text-wrapper">
            <span class="big-txt">Ładowanie...</span>
            <span class="status-txt">
                {{ current_name }}
            </span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { listen, Event as TauriEvent } from '@tauri-apps/api/event'
import { LoadingEvent } from '../types/LoadingEvent';

const current_name = ref("");
const is_loading = ref(true);

listen("loading", (evt: TauriEvent<LoadingEvent>) => {
    current_name.value = evt.payload.text || "";
    is_loading.value = evt.payload.is_loading;
});


</script>

<style scoped>
.loading-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.4);

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;

}

.text-wrapper {
    display: flex;
    align-items: flex-start;
    justify-content: center;
    flex-direction: column;
}

.big-txt {
    font-size: larger;
}

.status-txt {
    max-width: 200px;
    width: 200px;
    overflow: visible;
    white-space: nowrap;
    font-size: 0.8em;
}

.loading-circle {
    width: 30px;
    height: 30px;
    border: 5px solid transparent;
    
    border-right-color: white;
    border-radius: 50%;

    animation: loading-rotate 1s infinite linear;
}

@keyframes loading-rotate {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(359deg);
    }
}



</style>