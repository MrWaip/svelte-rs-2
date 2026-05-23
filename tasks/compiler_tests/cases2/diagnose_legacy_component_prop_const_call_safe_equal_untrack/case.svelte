<script>
    import Child from './child.svelte';
    export let x = 0;
    const tracker = { click: () => 1 };
</script>

<Child left={tracker.click()}>{x}</Child>
