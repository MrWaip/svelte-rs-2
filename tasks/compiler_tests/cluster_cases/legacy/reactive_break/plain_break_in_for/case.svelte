<script>
	let foo = 0;
	let bar = 0;
	$: {
		bar = 0;
		for (let i = 0; i < foo; i++) {
			if (i > 2) break;
			bar += i;
		}
	}
	foo = 5;
</script>

<h1>{foo} {bar}</h1>
