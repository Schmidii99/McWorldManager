<script lang="ts">
    import WorldDisplay from '$lib/WorldDisplay.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import { atobUTF8 } from '$lib/helper';

    let world_paths: string[] = [];

    onMount(async () => {
        let response: string = await invoke('get_saved_paths');
        // remove last empty item
        world_paths = response.split(',');
        
        for (let i = 0; i < world_paths.length; i++) {
            world_paths[i] = atobUTF8(world_paths[i]);
        }
    })
</script>

<div class="flex flex-wrap">
    {#each world_paths as path}
        <WorldDisplay path={path} />
    {/each}
</div>
