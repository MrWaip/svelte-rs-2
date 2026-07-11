<svelte:options runes={false} />

<script>
	let obj;
	function upd(e) {
		obj = e;
	}
</script>

<div style:width="{(obj?.w || 0) + 40}px" on:click={upd}></div>
