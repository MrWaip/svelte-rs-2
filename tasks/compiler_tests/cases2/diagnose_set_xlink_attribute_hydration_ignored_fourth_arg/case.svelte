<script>
	let { href } = $props();
</script>

<svg>
	<!-- svelte-ignore hydration_attribute_changed -->
	<a xlink:href={href}></a>
</svg>
