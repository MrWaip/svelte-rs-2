<script>
	import Widget from './Widget.svelte';
	let value = $state(5);
</script>

<Widget>
	{@const doubled = value * 2}
	<p>{doubled}</p>
</Widget>
