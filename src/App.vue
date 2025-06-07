<template>
  <div class="main-container">
    <TopBar @install-forge="installForgeOpen" @install-fabric="installFabric"/>
    <Content :available="available" :installed="installed"/>
  </div>
  <Loading />
  <InstallForge :visible="install_forge_visible" @close="installForgeClose"/>
</template>

<script setup lang="ts">
import InstallForge from './components/install/InstallForge.vue';
import Loading from './components/Loading.vue';
import TopBar from './components/main/TopBar.vue';
import Content from './components/main/Content.vue';
import { ref } from 'vue';
import { listen, Event as TauriEvent } from '@tauri-apps/api/event'
import { FolderContentsEvent } from './types/FolderContentsEvent';
import { ModFile } from './types/ModFile';
import { invoke } from '@tauri-apps/api';
import { ToastEvent } from './types/ToastEvent';

const available = ref([] as ModFile[]);
const installed = ref([] as ModFile[]);

const install_forge_visible = ref(false);

document.addEventListener('contextmenu', event => event.preventDefault());

function installForgeOpen() {
    install_forge_visible.value = true;
}

function installForgeClose() {
    install_forge_visible.value = false;
}

function installFabric() {
    invoke('install_fabric');
}

listen('folder_contents', function(evt: TauriEvent<FolderContentsEvent>) {
    const lists = evt.payload;
    available.value = lists.available;
    installed.value = lists.installed;
});

listen('toast', function(evt: TauriEvent<ToastEvent>) {
    alert(evt.payload.text)
})


</script>

<style scoped>
.main-container {
  display: flex;
  flex-direction: column;
  align-items: center;
}

</style>