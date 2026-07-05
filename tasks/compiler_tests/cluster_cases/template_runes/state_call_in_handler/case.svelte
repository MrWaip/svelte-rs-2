<script>
	import { SvelteSet } from 'svelte/reactivity';
	const set = new SvelteSet();
</script>

<button onclick={() => { const s = $state({ x: 1 }); set.add(s); }}>add</button>
