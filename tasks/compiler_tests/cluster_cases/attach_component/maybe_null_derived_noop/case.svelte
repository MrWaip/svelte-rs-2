<script>
	import Inner from './Inner.svelte';

	let { maybe } = $props();

	let handler = $derived(maybe ? (node) => {} : null);
</script>

<Inner {@attach handler} />
