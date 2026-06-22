<script>
	let foo = 0;
	let bar = 0;
	$: {
		bar = 0;
		for (let i = 0; i < foo; i++) {
			bar += i;
			if (i > 2) break $;
		}
	}
	foo = 5;
</script>

<h1>{foo} {bar}</h1>
