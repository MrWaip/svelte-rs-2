<script>
	export let container = {};
	export let paths = ['a'];
</script>
<div bind:this={container[paths[0]]}></div>
