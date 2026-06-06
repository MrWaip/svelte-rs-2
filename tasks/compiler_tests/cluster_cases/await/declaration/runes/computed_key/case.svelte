<script>
	const k = "z";
	let p = $state(Promise.resolve({ z: 1 }));
</script>
{#await p then { [k]: v }}
	<button>{v}</button>
{/await}
