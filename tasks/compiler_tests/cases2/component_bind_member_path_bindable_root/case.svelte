<script>
	import Comp from './Comp.svelte';
	let { store = $bindable() } = $props();
</script>

<Comp bind:value={store.inner.value} />
