<script>
	let show = true;
</script>

{#snippet co()}<b>C</b>{/snippet}
{#if show}{@render co()}{/if}
<select><option><span>M</span></option></select>
