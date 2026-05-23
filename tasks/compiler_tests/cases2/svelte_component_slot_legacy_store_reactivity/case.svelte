<svelte:options runes={false} />

<script>
    import { writable } from 'svelte/store';
    import Inner from './Inner.svelte';
    export let meta = writable({ hint: 'x' });
    let component = Inner;
</script>

<svelte:component this={component}>
    <span slot="caption">{$meta.hint || ''}</span>
</svelte:component>
