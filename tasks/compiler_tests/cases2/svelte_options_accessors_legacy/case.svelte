<svelte:options accessors={true} />
<script>
	export let count = 1;
	count = count + 5;
</script>

<p>{count}</p>
