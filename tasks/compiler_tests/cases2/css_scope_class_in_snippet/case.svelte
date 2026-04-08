<script>
	let variant = $state("primary");
</script>

{#snippet badge(text)}
	<span class="badge" class:primary={variant === "primary"}>{text}</span>
{/snippet}

{@render badge("hi")}

<style>
	.badge { color: red; }
	.primary { font-weight: bold; }
</style>
