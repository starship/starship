<template>
<div>
    <div v-for="prompt in prompts" :key="prompt.id">
        <h2>{{ prompt.name  }}</h2>
        <img :src="'/gallery/' + prompt.id + '.png'" :alt="prompt.name + ' Image'" @mouseover="hover = true" @mouseleave="hover = false" :class="{active: hover}" />
        <p>{{ prompt.description }}</p>
        <p>
            Created by <a :href="'https://github.com/' + prompt.githubId">{{ prompt.displayCreatorName }}</a>.
            Download the <a :href="prompt.link_raw" download>starship.toml</a> or view it on <a :href="prompt.link">GitHub</a>.
        </p>
        <!-- (please <em>right click</em> on the link and use <em>"Save Link As .../Download Linked File As ..."</em>). -->
    </div>
</div>
</template>

<script>
// import HoverImage from "../components/HoverImage";
import prompts from "../public/gallery.json";

export default {
    name: "Gallery",
    data() {
        return {
            prompts: prompts,
            hover: false,
        }
    }
}
</script>
