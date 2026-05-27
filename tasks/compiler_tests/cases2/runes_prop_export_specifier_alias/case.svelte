<script>
	let { count = 0 } = $props();
	export { count as foo };
</script>

<p>{count}</p>
