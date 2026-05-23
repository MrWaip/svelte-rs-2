<script>
    import { writable } from 'svelte/store';
    const state = writable({ x: 0 });
    let y = 0;
    $: y = $state.x;
</script>

{y}
