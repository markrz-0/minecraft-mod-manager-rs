<template>
    <div v-if="visible" class="modal-wrapper">
        <div class="modal__content">
            <button v-if="closable" class="modal__close-btn" @click="emit('close')">&#10006;</button>
            <slot></slot>
        </div>
    </div>
</template>

<script setup lang="ts">


const props = defineProps({
    visible: {
        type: Boolean,
        required: true
    },
    closable: {
        type: Boolean,
        default: true
    }
})

const emit = defineEmits(['close'])

</script>

<style scoped>
.modal-wrapper {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.4);

    display: flex;
    align-items: center;
    justify-content: center;
}

.modal__content {
    background-color:#2f2f2f;
    padding: 15px;
    border-radius: 5px;
    position: relative;
}


.modal__close-btn {
    background-color: transparent;
    border: none;

    top: 5px;
    right: 5px;
    position: absolute;
    color: white;
}

.modal__close-btn:hover {
    cursor: pointer;
}

</style>