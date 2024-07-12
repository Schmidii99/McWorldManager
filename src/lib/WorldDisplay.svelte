<script lang="ts">
    import { onMount } from "svelte";
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import { readDir, exists } from '@tauri-apps/api/fs';
    import { getVersionName } from "./MinecraftVersions";

    export let path: string;

    let img_url: string | null = null;
    let playerdata: any | null = null;

    onMount(async () => {
        if (await exists(path + "\\icon.png")) {
            img_url = convertFileSrc(path + "\\icon.png");
        }

        const firstPlayerDataFileName = (await readDir(path + "\\playerdata\\"))[0].name;
        
        let contents: string = await invoke("deserialize_nbt_file", {path: path + "\\playerdata\\" + firstPlayerDataFileName});
        
        playerdata = JSON.parse(contents);
    })
</script>

<div class="flex flex-col min-w-48 min-h-64 max-w-48 bg-teal-300 ml-4 mt-4 overflow-hidden">
    {#if img_url}
        <img class="w-48 h-48" src="{img_url}" alt="Thumbnail">
    {/if}
    <div class="textBox h-8 flex">
        <span>{path.split("\\").pop()}</span>
    </div>
    <div class="flex h-full items-end">
        {#if playerdata}
            <span class="text-gray-600">{getVersionName(playerdata["DataVersion"])}</span>
        {/if}
    </div>
</div>