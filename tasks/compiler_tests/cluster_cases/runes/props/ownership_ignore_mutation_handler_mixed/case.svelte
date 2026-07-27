<script>
	let { a, b } = $props();
</script>

<button
	onclick={() => {
		//svelte-ignore ownership_invalid_mutation
		a.x = 1;
		b.x = 2;
	}}
></button>
