<style>
	:global(.red) {
		color: red;
	}
	p :global(.child) {
		margin: 0;
	}
	p:global(.active) {
		font-weight: bold;
	}
	:global(h2) {
		font-size: 1.5em;
	}
	:global(.wrapper) :global(.item) {
		display: flex;
	}
	:global(h2.featured) {
		font-style: italic;
	}
	p {
		color: green;
	}
</style>
<p>content</p>
<p class="active">active</p>
<h2>heading</h2>
<h2 class="featured">featured</h2>
