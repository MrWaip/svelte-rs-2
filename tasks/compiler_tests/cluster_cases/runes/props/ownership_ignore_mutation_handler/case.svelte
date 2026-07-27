<script>
	let { test } = $props();
</script>

<button
	onclick={() => {
		//svelte-ignore ownership_invalid_mutation
		test.test = Math.random();
	}}
></button>
