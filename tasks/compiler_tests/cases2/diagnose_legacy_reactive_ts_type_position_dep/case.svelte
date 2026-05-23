<svelte:options runes={false} />

<script lang="ts">
    import data from './dep.js';
    let count = 0;
    $: doubled = { value: count } as Pick<typeof data, 'count'>;
</script>

{doubled.value}
