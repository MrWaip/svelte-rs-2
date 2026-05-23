<svelte:options runes={false} />
<script>
    import { state } from './store.js';
</script>

{#each $state.items as _item}
    <p>row</p>
{/each}
