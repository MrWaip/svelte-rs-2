<script>
	const foo = "hello";
</script>

<button onclick={() => {
	let foo = 0;
	function inner() {
		foo = 1;
	}
}}>click</button>
