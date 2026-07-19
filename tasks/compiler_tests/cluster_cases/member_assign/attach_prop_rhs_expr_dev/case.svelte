<script>
	let { opacity = 0.5 } = $props();
</script>

<div {@attach (node) => node.style.opacity = opacity}></div>
