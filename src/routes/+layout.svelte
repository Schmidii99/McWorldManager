<script lang="ts">
    import { onMount } from "svelte";
    import "../app.css";
    import { listen } from '@tauri-apps/api/event';
    import { message } from '@tauri-apps/api/dialog';
    
    let unlistenMessageboxError: null | any = null;
    let unlistenMessagebox: null | any = null;

    onMount(async () => {
        if (unlistenMessageboxError == null) {
            unlistenMessageboxError = await listen('messagebox_error', async (event: any) => {
                await message(event.payload["message"], { title: event.payload["title"], type: 'error' });
            });
        }
        if (unlistenMessagebox == null) {
            unlistenMessagebox = await listen('messagebox', async (event: any) => {
                console.log("Showing messagebox")
                await message(event.payload["message"], { title: event.payload["title"], type: 'info' });
            });
        }
        
    });

</script>
  
<slot />