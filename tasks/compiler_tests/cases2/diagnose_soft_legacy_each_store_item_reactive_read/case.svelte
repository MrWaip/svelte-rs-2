<svelte:options runes={false} />
<script>
    import { state } from './store.js';
</script>

{#each $state.items as item}
    <p>{item}</p>
{/each}
