<template>
    <Modal :visible="visible" @close="emit('close')">
        <div class="wrapper">
            <h2>Install Forge</h2>
            <span>Upewnij sie ze masz zainstalowana podstawowa wersje gry tej samej wersji</span>
            <div>
                <label for="mc-version">Wersja Minecraft:</label>
                <br />
                <select name="mc-version" @change="mcVersionChange($event)">
                    <option v-for="mc_version in mc_versions" :value="mc_version">{{ mc_version }}</option>
                </select>
                <br />
                <label for="mc-version">Wersja Forge:</label>
                <br />
                <select name="forge-version" @change="forgeVersionChange($event)">
                    <option v-for="forge_version in forge_versions" :value="forge_version">{{ forge_version }}</option>
                </select>
            </div>
            <button class="button" @click="download">Pobierz</button>
        </div>
    </Modal>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import Modal from '../Modal.vue';
import { ref } from 'vue';


const props = defineProps({
    visible: {
        type: Boolean,
        required: true
    }
})

const emit = defineEmits(['close']);

const mc_versions = ref([] as string[]);
const forge_versions = ref([] as string[]);

const selected_mc_version = ref("");
const selected_forge_version = ref("");

invoke('get_mc_versions').then((res) => {
    mc_versions.value = res as string[];
    selected_mc_version.value = mc_versions.value[0];
    getForgeVersion();
});

function mcVersionChange(event: any) {
    forge_versions.value = [];
    selected_forge_version.value = '';
    selected_mc_version.value = event.target.value;
    getForgeVersion();
}

function getForgeVersion() {
    invoke('get_forge_versions', {mcVersion: selected_mc_version.value} ).then((res) => {
        forge_versions.value = res as string[];
        selected_forge_version.value = forge_versions.value[0];
    });
}

function forgeVersionChange(event: any) {
    selected_forge_version.value = event.target.value;
}

function download() {
    if(selected_mc_version.value !== "" && selected_forge_version.value !== "") {
        invoke('install_forge', {mcVersion: selected_mc_version.value, forgeVersion: selected_forge_version.value});
        emit('close');
    }
}

</script>

<style scoped>
.wrapper {
    display: flex;
    flex-direction: column;
    /* align-items: ; */
    justify-content: center;
}

select {
    width: 100%;
    background-color: transparent;
    color: white;
}
option {
    background-color: #111;
}
</style>