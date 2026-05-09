<script>
    let { a, onClick, name } = $props();
</script>

<div class:x={a} class:y={Boolean(onClick)}>
    {name}
</div>
