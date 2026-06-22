<svelte:options runes={false} />
<script>
	function greet() {
		return 'hi';
	}
	export { greet };
</script>

<p>{greet()}</p>
