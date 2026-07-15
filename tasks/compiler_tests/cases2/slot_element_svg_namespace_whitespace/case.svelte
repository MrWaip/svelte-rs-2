<script>
	import Svg from './Svg.svelte';

	let paths = [];
	let polygons = [];
</script>

<Svg>
	<slot>
		{#each paths as path}
			<path {...path} />
		{/each}
		{#each polygons as polygon}
			<polygon {...polygon} />
		{/each}
	</slot>
</Svg>
