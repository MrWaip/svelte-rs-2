<script>
	let promise = Promise.resolve(() => {});
</script>

{#await promise then handler}
	<button onclick={handler}>x</button>
{/await}
