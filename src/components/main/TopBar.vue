<template>
    <div class="nav">
        <input v-model="val" class="nav__search-input" placeholder="szukaj" @keyup="emitChange">
        <button class="nav__btn button" v-bind:class="menu_shown ? 'toggled' : ''" @click="toggleMenu">
            <div>❯</div>
        </button>
        <div class="nav__menu-div"  v-bind:class="menu_shown ? 'shown' : ''">
            <button class="menu__btn" @click="importDialog">Importuj plik</button>
            <button class="menu__btn" @click="emit('installForge')">Zainstalluj Forge</button>
            <button class="menu__btn" @click="emit('installFabric')">Zainstalluj Fabric</button>
        </div>
    </div>
</template>

<script setup lang="ts">

import { ref, defineEmits } from 'vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';

const emit = defineEmits(['installForge', 'installFabric'])

const val = ref("");
const menu_shown = ref(false);

async function emitChange() {
    invoke("update_search_filter", {val: val.value.toLowerCase()}).then(() => {
        invoke('refresh_content');
    });
}

setTimeout(() => {
    emitChange()
}, 100);

async function importDialog() {
    const selected = await open({
        multiple: true,
        filters: [
            {
                name: "Java JARs",
                extensions: ["jar"]
            }
        ]
    });

    if (Array.isArray(selected)) {
        for(let file of selected) {
            await invoke('import_file', { filePath: file });
            await invoke('refresh_content');
        }
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        await invoke('import_file', { filePath: selected });
        await invoke('refresh_content');
    }
}

function toggleMenu() {
    menu_shown.value = !menu_shown.value;
}

</script>

<style scoped>
.nav {
  width: calc(100% - 42px);
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: none;
}

.nav__search-input {
  flex: 1;
}

.nav__search-input {
  border-radius: 8px;
  border: 2px solid transparent;
  margin: 2px 5px 5px 5px;
  padding: 0.6em 1em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #f6f6f6;
  background-color: #1f1f1f;
  outline: none;
}
.nav__btn > * {
    transform: rotate(-90deg);

}

.nav__btn.toggled > * {
    transform: rotate(90deg);
}

.nav__menu-div {
    display: none;
}

.nav__menu-div.shown {
    display: flex;
    flex-direction: column;

    background-color: #1f1f1f;
    border-radius: 5px;
    padding: 5px 0px;

    position: absolute;
    top: 3.8em;
    right: 1em;
}

.menu__btn {
    background-color: transparent;
    border: none;
    color: white;
    padding: 5px 10px;
    font-size: 1em;
}

.menu__btn:hover {
    background-color: #2a2a2a;
    cursor: pointer;
}


</style>