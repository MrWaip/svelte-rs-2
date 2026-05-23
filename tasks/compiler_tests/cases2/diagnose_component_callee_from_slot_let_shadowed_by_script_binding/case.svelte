<script>
	import Holder from './Holder.svelte';

	async function load() {
		const { default: Inner } = await import('./Inner.svelte');
		return Inner;
	}
</script>

<Holder task={load} let:value={Inner}>
	<Inner />
</Holder>
