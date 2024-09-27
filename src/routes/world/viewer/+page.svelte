<script lang="ts">
    import { goto } from "$app/navigation";
    import { currentWorldPathStore } from "$lib/stores";
</script>

<div class="flex w-full items-center justify-around">
    <button class="hover:bg-gray-400 hover:cursor-pointer w-full text-center" on:click={async () => {
        await goto("/world/");
    }}>Overview</button>
    <button class="hover:bg-gray-400 hover:cursor-pointer w-full text-center" on:click={async () => {
        await goto("/world/players");
    }}>Players</button>
    <button class="underline w-full text-center">World</button>
</div>

<div>World viewer</div>