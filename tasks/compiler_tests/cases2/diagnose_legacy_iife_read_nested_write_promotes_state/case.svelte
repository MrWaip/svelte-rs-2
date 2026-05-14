<script>
    import { onMount } from 'svelte';
    let flag = false;
    $: derived_count = (() => {
        if (flag) {
            return 1;
        }
        return 0;
    })();
    onMount(() => {
        const cb = () => { flag = true; };
        cb();
    });
</script>
<p>{derived_count}</p>
