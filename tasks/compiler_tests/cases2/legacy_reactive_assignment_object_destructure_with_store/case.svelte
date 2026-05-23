<script>
    export let source;

    $: ({ left, right, alias: renamed, nested: { deep } } = source);
</script>

<p>{$left}-{$right}-{renamed}-{deep}</p>
