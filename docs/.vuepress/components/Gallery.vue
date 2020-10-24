<template>
  <div>
    <div v-for="prompt in prompts" :key="prompt.githubId">
      <h3><a :href="'https://github.com/' + prompt.githubId">{{ prompt.githubId }}</a></h3>
      <p>
        View <code>starship.toml</code> of below prompt on <a :href="'https://github.com/starship/starship/blob/master/docs/.vuepress/public/gallery/' + prompt.githubId + '/starship.toml'">GitHub</a>.
      </p>
      <img :src="'/gallery/' + prompt.githubId + '/prompt.png'" alt="Prompt Image"/>
    </div>
  </div>
</template>

<script>
  import prompts from "../components/prompts"

  export default {
    name: "Gallery",
    data() {
      return {
        prompts: prompts,
      }
    }
  }
</script>