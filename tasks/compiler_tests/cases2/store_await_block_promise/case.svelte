<script>
	import { promise } from './stores';
</script>

{#await $promise}<p>p</p>{:then value}<p>{value}</p>{/await}
