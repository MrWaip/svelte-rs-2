<script>
	import Foo from './Foo.svelte';
</script>

<Foo let:onClick>
	<p>{onClick}</p>
</Foo>
