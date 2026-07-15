<svelte:options runes={false} />

<script>
	export let adapter;
	export let day;
	let focused = null;
	function pick() {
		focused = day;
	}
</script>

<button autofocus={focused !== null && adapter.isSame(day, focused)} on:click={pick}></button>
