<script>
	export let label = 'sum';

	let a = 1;
	let b = 2;

	$: console.log(`${label}: ${sum}`);
	$: sum = a + b;
	$: ((param) => {
		via_iife = param * 2;
	})(sum);
</script>

<p>{sum}-{via_iife}</p>
