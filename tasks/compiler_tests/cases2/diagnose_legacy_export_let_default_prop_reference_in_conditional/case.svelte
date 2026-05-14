<svelte:options runes={false} />

<script>
    export let kind = 'a';
    export let label = kind === 'a' ? 'first' : 'second';
</script>

<div>{label}</div>
