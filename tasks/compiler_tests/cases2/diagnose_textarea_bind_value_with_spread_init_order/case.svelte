<script>
    let { value = $bindable(""), extra = {} } = $props();
</script>

<textarea bind:value {...extra} />
