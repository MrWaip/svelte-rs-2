<style>
	.a :global .b .c {
		color: red;
	}

	section :global strong {
		font-weight: bold;
	}

	:global .page .title {
		margin: 0;
	}
</style>

<div class="a">
	<div class="b">
		<div class="c">compound</div>
	</div>
</div>

<section>
	<strong>bare global</strong>
</section>

<h1 class="title">title</h1>
