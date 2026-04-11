<style>
	[data-state="on"] {
		color: red;
	}

	[data-state="on" i] {
		background: blue;
	}

	[data-state="on" s] {
		border-color: green;
	}

	[type="BUTTON"] {
		color: purple;
	}
</style>

<div data-state="on">exact</div>
<div data-state="On">insensitive</div>
<div data-state="off">off</div>
<button type="button" aria-label="run">button</button>
