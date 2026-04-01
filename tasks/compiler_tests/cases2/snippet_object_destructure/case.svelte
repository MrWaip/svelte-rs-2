<script>
	let data = $state({ name: "world", age: 25 });
</script>

{#snippet greeting({ name, age })}
	<p>{name} is {age}</p>
{/snippet}

{#snippet withDefault({ label = "default" })}
	<span>{label}</span>
{/snippet}

{#snippet withRest({ id, ...rest })}
	<div>{id}</div>
{/snippet}

{@render greeting(data)}
{@render withDefault({})}
{@render withRest({ id: 1, extra: true })}
