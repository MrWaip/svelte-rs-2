<svelte:options customElement="my-el" />

<script>
	export let name;
</script>

<p>name: {name}</p>
<p>{JSON.stringify($$restProps)}</p>
