<script>
	import Foo from './Foo.svelte';
</script>

<Foo>
	<span>default</span>
	<svelte:element this="span" slot='other'>This is other slot</svelte:element>
</Foo>
