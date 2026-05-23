<script>
    import { writable } from 'svelte/store';
    const w = writable(0);
    let derived = 0;
    $: derived = $w * 2;
    $: (() => { $w = 1; })();
</script>
<p>{derived}</p>
