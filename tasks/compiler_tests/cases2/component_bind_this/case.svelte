<script>
	import Component from './Component.svelte';
	let ref = $state();
	let plainRef;
</script>

<Component bind:this={ref} />
<Component bind:this={plainRef} />
<Component bind:this={ref} name="test">
	<p>child content</p>
</Component>
