<template>
  <div>
    <div v-for="prompt in prompts" :key="prompt.githubId">
      <h3><a :href="'https://github.com/' + prompt.githubId">{{ prompt.displayName }}</a></h3>
      <div v-for="preset in prompt.presets" :key="preset.id">
        <h4>{{preset.name}}</h4>
        <img :src="'/gallery/' + prompt.githubId + '/' + preset.id + '/prompt.png'" :alt="preset.name + 'Image'"/>
        <p v-if="preset.info.length != 0">
          {{preset.info}}
        </p>
        <p v-if="preset.external">
          Download the <a :href="preset.external" download>starship.toml</a> (please <em>right click</em> on the link and use <em>"Save Link As .../Download Linked File As ..."</em>).
        </p>
        <p v-else>
          Download the <a :href="'/gallery/' + prompt.githubId +  '/' + preset.id + '/starship.toml'">starship.toml</a>.
        </p>
      </div>
      <div v-if="prompt.presets.length == 0">
        <img :src="'/gallery/' + prompt.githubId + '/prompt.png'" alt="Prompt Image"/>
        <p>
          Download the <a :href="'/gallery/' + prompt.githubId + '/starship.toml'">starship.toml</a>.
        </p>
      </div>
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
