<script setup lang="ts">
    import { useFigureNumber } from './figure-count.ts'


    const props = defineProps<{
        caption?: string
        src: string
    }>()

    const figureNumber = useFigureNumber()

</script>

<template>
    <figure>
        <img :src="src" />
        <figcaption>Fig.{{ figureNumber }}{{ caption ? ` - ${caption}` : '' }}</figcaption>
    </figure>
</template>
