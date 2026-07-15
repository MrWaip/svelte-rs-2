<script>
	import Comp from './Comp.svelte';
	let { store } = $props();
</script>

<Comp bind:value={store.inner.value} />
