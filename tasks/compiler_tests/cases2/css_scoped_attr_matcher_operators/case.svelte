<style>
	[data-tags~="active"] {
		color: red;
	}

	[data-lang|="en"] {
		color: blue;
	}

	[data-url^="https://"] {
		color: green;
	}

	[data-url$=".com"] {
		color: orange;
	}

	[data-url*="example"] {
		color: purple;
	}
</style>

<div data-tags="card active">class</div>
<div data-lang="en-US">lang</div>
<div data-url="https://example.com">href</div>
<span data-tags="inactive">no class</span>
<div data-lang="bengali">no lang</div>
<div data-url="http://sample.org">no href</div>
