<script lang="ts">
    import WorldDisplay from '$lib/WorldDisplay.svelte';
import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let world_urls: string[] = [];

    onMount(async () => {
        let response: string = await invoke('get_saves');
        // remove last empty item
        world_urls = response.split(',').slice(0, -1);
        
        for (let i = 0; i < world_urls.length; i++) {
            world_urls[i] = atob(world_urls[i]);
        }
    })
</script>

<div class="flex flex-wrap">
    {#each world_urls as url}
        <WorldDisplay path={url} />
    {/each}
</div>
