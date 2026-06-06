<script>
	let promise = Promise.reject();
</script>

{#await promise catch handler}
	<button onclick={handler}>x</button>
{/await}
