<script>
    let { value = $bindable("") } = $props();
    let group = $state([]);
</script>

<input type="radio" bind:group={group} value="a" />
<input type="radio" bind:group={group} value="b" />
<p>{value}</p>
