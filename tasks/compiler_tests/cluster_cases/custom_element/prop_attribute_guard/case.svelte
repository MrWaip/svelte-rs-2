<svelte:options customElement={{ tag: "x-attr", props: { foo: { attribute: "foo-bar" } } }} />

<script>
	export let foo;
	foo;
</script>

<div>hi</div>
