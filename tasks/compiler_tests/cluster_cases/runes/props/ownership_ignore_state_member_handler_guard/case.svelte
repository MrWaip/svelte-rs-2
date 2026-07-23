<script>
	let s = $state({ x: 0 });
</script>

<button
	onclick={() => {
		//svelte-ignore ownership_invalid_mutation
		s.x = 1;
	}}
></button>
