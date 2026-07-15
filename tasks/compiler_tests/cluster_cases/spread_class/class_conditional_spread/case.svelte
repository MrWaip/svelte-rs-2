<script>
	export let foo;
	export let a;
	export let b;
	export let attributes = {};
</script>

<div class={foo ? a : b} {...attributes}></div>
