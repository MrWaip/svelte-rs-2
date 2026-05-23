<script>
    export let store;
    $: ({ value } = store);
</script>

<p>{$value}</p>
