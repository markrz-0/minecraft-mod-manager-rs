<script setup lang="ts">
import { ModFile } from '../../types/ModFile';
import Item from './Item.vue';

const props = defineProps({
    item: {
        type: Object as () => ModFile,
        required: true
    }
})

</script>

<template>
    <Item :listname="'available'" :item="props.item"/>
</template>