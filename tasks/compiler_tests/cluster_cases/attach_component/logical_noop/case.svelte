<script>
	import Inner from './Inner.svelte';

	function attachment() {
		console.log('up');
	}

	let enabled = $state(false);
</script>

<button onclick={() => enabled = !enabled}></button>

<Inner {@attach enabled && attachment} />
