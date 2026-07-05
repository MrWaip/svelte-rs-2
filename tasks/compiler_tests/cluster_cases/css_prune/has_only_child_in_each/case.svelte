<script>
	let items = [1];
</script>

<div class="a">
	{#each items as x}
		<div class="b">{x}</div>
	{/each}
</div>

<style>
	.a:has(.b:only-child) {
		color: red;
	}
</style>
