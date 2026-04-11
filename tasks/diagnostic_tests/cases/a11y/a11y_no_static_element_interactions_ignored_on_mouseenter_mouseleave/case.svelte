<script>
	let hovered = $state(false);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	onmouseenter={() => (hovered = true)}
	onmouseleave={() => (hovered = false)}
	style="padding:1em;border:1px solid #ddd;border-radius:8px;width:fit-content;"
>
	{#if hovered}
		<span>👋 You are hovering!</span>
	{:else}
		<span>Hover over me</span>
	{/if}
</div>
