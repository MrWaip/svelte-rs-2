<script>
	import Comp from './Comp.svelte';
	let comp = $state();
</script>
<Comp bind:this={() => comp, v => comp = v}/>
