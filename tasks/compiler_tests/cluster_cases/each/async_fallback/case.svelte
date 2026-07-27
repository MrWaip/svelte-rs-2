<script>
	let n = $state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => n++}>inc</button>

{#each await delay([n]) as item}
	<p>{item}</p>
{:else}
	<p>empty</p>
{/each}
