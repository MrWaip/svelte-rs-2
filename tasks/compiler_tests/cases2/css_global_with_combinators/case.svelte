<div class="cell-block">
	<div class="cell">a</div>
	<div class="cell"><div class="content">b</div></div>
</div>

<style>
	.cell-block :global(.cell + .cell > .content) {
		border-top: 1px solid red;
	}

	.cell-block :global(.x ~ .y) {
		color: blue;
	}
</style>
