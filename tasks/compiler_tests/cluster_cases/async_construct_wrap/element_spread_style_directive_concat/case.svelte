<script>
	async function g() { return 1 }
</script>
<div {...{q:1}} style:color="{await g()}px"></div>
