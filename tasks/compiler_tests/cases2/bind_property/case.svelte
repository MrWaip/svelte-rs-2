<script>
	let indeterminate = $state(false);
	let open = $state(true);
</script>

<input type="checkbox" bind:indeterminate />

<details bind:open></details>
