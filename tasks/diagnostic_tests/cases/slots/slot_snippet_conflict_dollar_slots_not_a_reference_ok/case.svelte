<script>
	let { children } = $props();
	const bag = { $$slots: 1 };
	const picked = bag.$$slots;
</script>

<p>{picked}</p>
{@render children?.()}
