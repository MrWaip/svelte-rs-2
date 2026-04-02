<script>
	let data = $state({
		nested: { name: "world" },
		list: [[10, 20, 30]],
		meta: { note: "ok" }
	});
</script>

{#snippet view({ nested: { name = "fallback" }, list: [[first, ...rest]], ...tail })}
	<p>{name} {first} {rest.length} {tail.meta.note}</p>
{/snippet}

{@render view(data)}
