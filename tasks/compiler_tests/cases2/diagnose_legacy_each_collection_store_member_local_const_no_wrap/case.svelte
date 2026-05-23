<script>
    import { writable } from 'svelte/store';
    function makeAdapter() {
        return { state: writable({ items: [1, 2, 3] }) };
    }
    const { state } = makeAdapter();
</script>

{#each $state.items as item}
    <p>{item}</p>
{/each}
