<script>
	import Comp from './Comp.svelte';
	let store = $state({ inner: { value: 0 } });
</script>

<Comp bind:value={store.inner.value} />
