<script>
	export let variant;
</script>

<button {...$$props} class="variant-{variant} {$$props.class ?? ''}">
	click me
</button>
