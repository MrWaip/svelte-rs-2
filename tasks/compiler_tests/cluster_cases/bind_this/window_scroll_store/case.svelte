<script>
	import { writable } from 'svelte/store';
	const y = writable(0);
</script>
<svelte:window bind:scrollY={$y}/>
<p>{$y}</p>
