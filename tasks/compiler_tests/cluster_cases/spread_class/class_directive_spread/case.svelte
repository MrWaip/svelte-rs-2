<script>
	export let myClass;
	export let flag;
	export let attributes = {};
</script>

<div class={myClass} class:on={flag} {...attributes}></div>
