<svelte:options runes={false} />
<script>
    import { pick } from './pick';
    import { count } from './count';
    export let kind;
</script>

{#each pick(kind, $count) as item}
    <div>{item}</div>
{/each}
