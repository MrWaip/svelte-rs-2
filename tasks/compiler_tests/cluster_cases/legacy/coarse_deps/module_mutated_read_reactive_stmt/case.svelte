<script module>
	let count = 0;
</script>
<script>
	export function bump() {
		count = count + 1;
	}

	$: doubled = count;
</script>

<p>{doubled}</p>
