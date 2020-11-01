<template>
<div>
    <div v-for="prompt in prompts" :key="prompt.id">
        <h2>{{ prompt.name  }}</h2>
        <HoverImage :prompt="prompt" />
    </div>
</div>
</template>

<script>
import HoverImage from "../components/HoverImage";
import prompts from "../public/gallery.json";

export default {
    name: "Gallery",
    data() {
        return {
            prompts: prompts,
        }
    }
}
</script>
