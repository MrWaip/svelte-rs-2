<script>
	export let a;
	export function foo() {}
	$: {
		a;
		foo();
	}
</script>
