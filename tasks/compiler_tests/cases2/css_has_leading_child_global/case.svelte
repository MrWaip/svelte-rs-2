<div class="a">x</div>
<style>
	.a {
		&:has(> :global(.b)) { color: red; }
	}
</style>