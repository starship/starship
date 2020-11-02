<template>
<div>
    <div @mouseover="hover=true" @mouseleave="hover = false" class="prompt">
        <img class="image" :src="'/gallery/' + prompt.id + '.png'" :alt="prompt.name + ' Image'" />
        <transition name="buttons">
            <div class="buttons" v-if="hover">
                <span v-on:click="downloadText(prompt.link_raw, $event)" class="action-button gallery-button">
                    Download the starship.toml
                </span>
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
    },
    methods: {
        downloadText: function(url, event) {
            fetch(url, {
                method: "GET",
                headers: {
                    "Content-Type": "text/plain",
                },
            })
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`${res.status} ${res.statusText}`);
                }
                return res.blob();
            })
            .then((blob) => {
                // create <a> element for blob
                const url = URL.createObjectURL(blob);
                const a = document.createElement("a");
                document.body.appendChild(a);
                a.download = "starship.toml"
                a.href = url;

                // simulate clicking download button
                a.click();
                a.remove();
                return true;
            })
            .then((isDownloadDone) => {
                URL.revokeObjectURL(url);
            })
            .catch((reason) => {
                console.log(reason);
            });
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
    filter: brightness(50%);
}

/* buttons */ 
.buttons {
    position: absolute;  /* for putting buttons on image */
    right: 1.5rem;
    bottom: 2rem;
    opacity: 0.9;
}

.buttons a, span {
    color: white;
    font-size: 1rem;
    font-weight: 500;
    margin-left: 1rem;
}

.buttons a:hover {
    text-decoration: none;
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
    .buttons a, span {
        display: block;
        margin-top: 0.5rem;
    }
}
</style>
