<svelte:options runes={false} />

<script>
    import { writable } from 'svelte/store';
    import Inner from './Inner.svelte';
    export let meta = writable({ disabled: false });
    let x;
    let component = Inner;
</script>

<svelte:component this={component}>
    <div slot="icon">
        {#if x && !$meta.disabled}
            <div>hi</div>
        {/if}
    </div>
</svelte:component>
