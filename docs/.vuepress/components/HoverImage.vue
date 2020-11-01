<template>
<div>
    <div @mouseover="hover=true" @mouseleave="hover = false" class="prompt">
        <img class="image" :src="'/gallery/' + prompt.id + '.png'" :alt="prompt.name + ' Image'" />
        <transition name="buttons">
            <div class="buttons" v-if="hover">
                <a :href="prompt.link_raw" class="action-button gallery-button" download>
                    Download the starship.toml
                </a>
                <a :href="prompt.link" target="_black" class="action-button gallery-button">
                    View on GitHub
                </a>
            </div>
        </transition>
    </div>
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
/* prompt */ 
.prompt {
    position: relative;  /* for putting buttons on image */
}

.prompt img {
    transition: 0.75s all;
}

.prompt:hover img {
    filter: brightness(60%);
}

/* buttons */ 
.buttons {
    position: absolute;  /* for putting buttons on image */
    right: 1.5rem;
    bottom: 2rem;
    opacity: 0.9;
}

.buttons a {
    color: white;
    font-size: 1rem;
    font-weight: 500;
    margin-left: 0.5rem;
}

.buttons a:hover {
    text-decoration: none;
    position: relative;
}

.gallery-button {
    border-style: none;
    border-radius: 4px;
    padding: 0.3rem 0.6rem;
    cursor: pointer;
}

/* transition of "buttons" */
.buttons-enter-active {
    transition: all 0.5s ease;
}

.buttons-leave-active {
    transition: all 0.5s cubic-bezier(1.0, 0.5, 0.8, 1.0);
    /* transition: all 0.75s cubic-bezier(1.0, 0.5, 0.8, 1.0);*/
}

.buttons-enter, .buttons-leave-to {
    transform: translateY(5px);
    opacity: 0;
}

@media (max-width: 480px) {
    .buttons a {
        display: block;
        margin-top: 0.5rem;
    }
}
</style>
