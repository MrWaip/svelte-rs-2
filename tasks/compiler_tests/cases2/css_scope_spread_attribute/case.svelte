<script>
	let { ...rest } = $props();
</script>

<p {...rest} data-extra="x">spread</p>

<style>
	p { color: green; }
</style>
