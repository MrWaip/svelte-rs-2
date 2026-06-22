<script>
	import Foo from './Foo.svelte';
	import Bar from './Bar.svelte';
	export let x;
</script>
<svelte:component this="{x ? Foo : Bar}" answer={42}>
	<span>child</span>
</svelte:component>
