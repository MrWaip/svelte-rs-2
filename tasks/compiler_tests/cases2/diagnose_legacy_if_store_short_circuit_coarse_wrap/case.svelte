<script>
    import { writable } from 'svelte/store';

    export let prop;
    const items = writable([]);
    let show = true;
</script>

{#if show && $items.length > 0}
    <p>shown {prop}</p>
{/if}
