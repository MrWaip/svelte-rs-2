<script>
	let { obj } = $props();
	let n = $state(0);
</script>
<button onclick={() => { n++; obj.run(); }}>{n}</button>
