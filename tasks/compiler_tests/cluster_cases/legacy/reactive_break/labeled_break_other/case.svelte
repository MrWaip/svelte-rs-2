<script>
	let foo = 0;
	let bar = 0;
	$: {
		bar = 0;
		outer: for (let i = 0; i < foo; i++) {
			for (let j = 0; j < foo; j++) {
				if (j > i) break outer;
				bar += j;
			}
		}
	}
	foo = 4;
</script>

<h1>{foo} {bar}</h1>
