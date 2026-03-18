<script>
	import Component from './Component.svelte';
	let ref = $state();
</script>

<Component bind:this={ref} name="test">
	<p>child content</p>
</Component>
