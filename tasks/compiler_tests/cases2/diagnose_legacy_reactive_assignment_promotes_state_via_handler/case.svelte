<svelte:options runes={false} />

<script>
	export let trigger;

	let value;

	$: value = trigger;

	function read() {
		return value;
	}
</script>

<button on:click={read}></button>
