<template>
<div>
    <img :src="'/gallery/' + prompt.id + '.png'" :alt="prompt.name + ' Image'" @mouseover="hover = true" @mouseleave="hover = false" :class="{active: hover}" />
    <p v-show="hover">Mouceovered!!!</p>
</div>
</template>

<script>
export default {
    props: ["prompt"],
    data() {
        return {
            hover: false,
        }
    }
}
</script>

<style scoped>
.active {
    border: 2px solid red;
}
</style>
