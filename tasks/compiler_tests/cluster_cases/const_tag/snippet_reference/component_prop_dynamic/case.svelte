<script>
	let { x } = $props();
</script>
<Comp>
	{@const foo = x * 2}
	{#snippet prop()}{foo}{/snippet}
</Comp>
