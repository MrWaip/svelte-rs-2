<svelte:options runes={false} />
<script>
    import { items } from './data';
    export let foo;
</script>

{#each items() as item}
    <div>{item}</div>
{/each}
