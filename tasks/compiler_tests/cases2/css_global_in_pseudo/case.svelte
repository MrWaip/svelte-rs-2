<style>
	p:not(:global(.active)) {
		color: red;
	}
	:is(:global(.foo), .bar) {
		font-weight: bold;
	}
	div:where(:global(.x)) {
		margin: 0;
	}
</style>
<p>content</p>
<p class="bar">bar</p>
<div>box</div>
