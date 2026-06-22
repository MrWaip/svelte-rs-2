<script>
	export let a;
	function foo() {}
	$: {
		a;
		foo();
	}
</script>
