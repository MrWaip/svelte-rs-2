<script lang="ts">
	let obj = $state<{ field: { x: number } } | null>({ field: { x: 0 } });
	let deep = $state<{ a: { b: { c: { x: number } } } | null } | null>({ a: { b: { c: { x: 0 } } } });

	function script_ops() {
		obj!.field.x = 1;
		obj!.field.x += 2;
		obj!.field.x++;
		++obj!.field.x;
		obj!.field.x &&= 3;

		deep!.a!.b.c.x = 4;
		deep!.a!.b.c.x += 5;
		deep!.a!.b.c.x++;
	}
</script>

{obj!.field.x}
{deep!.a!.b.c.x}

{obj!.field.x = 1}
{obj!.field.x += 2}
{obj!.field.x++}
{++obj!.field.x}

{deep!.a!.b.c.x = 4}
{deep!.a!.b.c.x++}

<button onclick={script_ops}>run</button>
