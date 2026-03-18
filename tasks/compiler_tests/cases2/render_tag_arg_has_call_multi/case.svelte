<script>
	function fn1() { return 'a'; }
	function fn2() { return 'b'; }
</script>

{#snippet show(a, b)}
	<p>{a} {b}</p>
{/snippet}

{@render show(fn1(), fn2())}