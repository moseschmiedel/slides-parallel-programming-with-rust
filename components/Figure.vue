<script setup lang="ts">
    import { useFigureNumber } from './figure-count.ts'


    const props = defineProps<{
        caption?: string
        src: string
        number: number
    }>()

    const figureNumber = useFigureNumber()

</script>

<template>
    <figure>
        <img :src="src" />
        <figcaption>Fig.{{ number }}{{ caption ? ` - ${caption}` : '' }}</figcaption>
    </figure>
</template>
