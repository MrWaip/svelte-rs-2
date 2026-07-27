<script>
	let { p } = $props();
	let a = await p;
	function f() { return 1; }
	let b = 2;
</script>
{a}{b}{f()}
