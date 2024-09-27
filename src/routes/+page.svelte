<script lang="ts">
    import WorldDisplay from '$lib/WorldDisplay.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onDestroy, onMount } from 'svelte';
    import { atobUTF8 } from '$lib/helper'
    import { listen } from '@tauri-apps/api/event';

    let world_paths: string[] = [];

    let unlistenRefresh: any = null;

    onMount(async () => {
        await loadWorlds();

        unlistenRefresh = await listen('refresh', (event) => {
            loadWorlds();
        });
    })
    onDestroy(() => {
        if (unlistenRefresh == null) { return; }
        unlistenRefresh();
    })


    async function loadWorlds() {
        let response: string = await invoke('get_saved_paths');
        world_paths = response.split(',');
        
        for (let i = 0; i < world_paths.length; i++) {
            world_paths[i] = atobUTF8(world_paths[i]);
        }
    }
</script>

<div class="flex flex-wrap">
    {#each world_paths as path}
        <WorldDisplay path={path} />
    {/each}
</div>
