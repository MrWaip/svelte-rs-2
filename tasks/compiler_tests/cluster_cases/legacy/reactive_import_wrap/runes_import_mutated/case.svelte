<script>
	import foo from './foo.js';
	function add() {
		foo.bar = 'baz';
	}
</script>

<p>{foo.bar}</p>
<button onclick={add}>add</button>
