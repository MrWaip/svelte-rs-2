<script>
	import Child from './Child.svelte';
</script>

<Child>
	<div class="a" slot="x">a</div>
	<div class="b" slot="y">b</div>
</Child>
<div class="c">c</div>

<style>
	.a + .c { color: green; }
	.b + .c { color: green; }
</style>
