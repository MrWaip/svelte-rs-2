<script>
	import Inner from './Inner.svelte';

	let enabled = $state(true);
	let handler = $derived(enabled ? (node) => {} : null);
</script>

<Inner {@attach handler} prop="value" />
