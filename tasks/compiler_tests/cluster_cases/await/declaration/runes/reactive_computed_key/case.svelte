<script>
	let { object } = $props();
	let num = $state(0);
</script>
{#await object then { [`k${num++}`]: v }}
	<button>{v} {num}</button>
{/await}
