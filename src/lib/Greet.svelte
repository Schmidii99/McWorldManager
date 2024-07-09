<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let name = ''
    let greetMsg = ''

    async function greet() {
        greetMsg = await invoke('greet', { name })
    }
</script>

<div class="flex justify-center mt-3">
    <input class=" mx-3 rounded border border-solid border-gray-700" id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button class="hover:bg-slate-400 bg-slate-300 rounded px-3" on:click="{greet}">Greet</button>
    <p>{greetMsg}</p>
</div>