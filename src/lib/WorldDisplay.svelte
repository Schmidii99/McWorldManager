<script lang="ts">
    import { onMount } from "svelte";
    import { convertFileSrc } from '@tauri-apps/api/tauri';

    export let path: string;

    let img_url = "";

    onMount(() => {
        console.log("Trying to load: " + path);
        img_url = convertFileSrc(path + "/icon.png");
    })
</script>

<div class="flex flex-col min-w-48 min-h-64 max-w-48 max-h-64 bg-teal-300 ml-4 mt-4 overflow-hidden">
    <img class="w-48 h-48" src="{img_url}" alt="">
    <div class="textBox h-8 flex overflow-hidden">
        <span>{path.split("\\").pop()}</span>
    </div>
</div>