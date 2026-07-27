<svelte:options runes={false} />

<script>
	function outer() {
		let $$a = 1;
		var $$b = 2;
		function $$inner() {}
		return [$$a, $$b, $$inner];
	}
</script>

{outer}
