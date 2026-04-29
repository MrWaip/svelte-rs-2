<svelte:options runes={false} />
<script context="module">
	let shared = 0;
	export function bump() { shared += 1; }
</script>
<script>
	$: total = shared * 2;
</script>

<p>{total}</p>
