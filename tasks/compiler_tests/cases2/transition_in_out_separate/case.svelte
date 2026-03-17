<script>
	import { fly, fade } from 'svelte/transition';
</script>

<div in:fly={{ y: 200 }} out:fade>hello</div>
