<script>
	const promise = fetch('/api');
</script>

{#await promise then { name, age }}
	<p>{name} is {age}</p>
{/await}
