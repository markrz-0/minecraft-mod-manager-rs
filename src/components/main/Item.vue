<template>
    <div class="item" @click="moveItem" @auxclick="remove">
        <div class="user-select-none">
            <span v-if="props.listname === 'available'">&larr;</span>
        </div>
        <div class="item__content">
            <div class="item__content__text">
                <span v-if="!props.item.is_jar" class="item-error">&#9888;</span>
                <span>{{ props.item.name }}</span>
            </div>
            <div class="item__content__versions" v-if="props.item.is_jar">
                <span class="item__content__versions__span">{{ props.item.loader || 'Loader?' }}</span>
                <span class="item__content__versions__span">{{ props.item.mc_version || 'Version?'}}</span>
            </div>
        </div>
        <div class="user-select-none">
            <span v-if="props.listname === 'installed'">&rarr;</span>
        </div> 
    </div>
</template>


<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { ask } from '@tauri-apps/api/dialog';
import { ModFile } from '../../types/ModFile';


const props = defineProps({
    item: {
        type: Object as () => ModFile,
        required: true
    },
    listname: {
        type: String,
        required: true 
    },
});

async function moveItem() {
    const otherListname = props.listname === 'available' ? 'installed' : 'available';
    await invoke('move_file', {file: props.item.name, to: otherListname});
    await invoke('refresh_content');
}

async function remove() {
    const ans = await ask(`Czy na pewno chcesz usunać ${props.item.name} ?`, "Usuń");
    if(ans) {
        await invoke('del_file', {file: props.item.name, from: props.listname});
        await invoke('refresh_content');
    }
}

</script>

<style scoped>
.item-error {
  color: yellow;
  font-size: large;
  font-style: bold;
}

.item {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  width: 90%;
  border-radius: 8px;
  border: 2px solid transparent;
  margin: 2px 5px 5px 5px;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #f6f6f6;
  background-color: #1f1f1f;
}

.item {
  align-items: center;
  text-align: center;
  box-shadow: 0px 5px 2px rgba(0, 0, 0, 0.2);
}

.item:active {
  box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.2);
  margin-top: 5px;
  margin-bottom: 2px;
}

.item {
  cursor: pointer;
}

.item__content {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 5px;
}

.item__content__text {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.item__content__versions {
  width: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-around;
  gap: 10px;
}

.item__content__versions__span {
  color: #777;
  font-style: italic;
  font-size: smaller;
  flex: 1;
}


@media screen and (max-width: 570px) {
  .item {
    width: 85%;
  }
}

@media screen and (max-width: 540px) {
  .item {
    width: 80%;
  }
}
</style>