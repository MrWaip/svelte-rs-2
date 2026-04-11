<script>
	import { fade } from 'svelte/transition';

	async function load() {
		return { duration: 100 };
	}
</script>

<div transition:fade={await load()}></div>
