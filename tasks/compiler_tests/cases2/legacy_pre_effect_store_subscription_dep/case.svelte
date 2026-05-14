<svelte:options runes={false} />

<script>
    import { writable } from 'svelte/store';
    function pick(value) {
        return value;
    }
    const source = writable(0);
    $: actions = pick($source);
</script>

{actions}
