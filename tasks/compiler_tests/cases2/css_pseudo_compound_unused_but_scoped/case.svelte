<style>
	:is(.foo).bar {
		color: red;
	}

	:where(.baz)[data-nope] {
		color: blue;
	}
</style>

<p class="foo">foo</p>
<span class="baz">baz</span>
