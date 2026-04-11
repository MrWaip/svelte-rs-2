<svelte:options css="injected" />

<style>
	.used {
		color: red;
	}

	.unused {
		color: blue;
	}

	.used,
	.unused-mixed {
		border: 1px solid;
	}
</style>

<div class="used">used</div>
