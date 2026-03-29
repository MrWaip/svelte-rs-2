<script>
    let width = $state(0);
    let content = $state("");
</script>

<div bind:clientWidth={width} contenteditable bind:innerHTML={content}>editable</div>
