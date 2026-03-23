<script lang="ts">
	import type { SomeType } from './types';
	import { type AnotherType, realValue } from './utils';

	let data: SomeType = $state({ value: 0 });

	function process(input: AnotherType) {
		return realValue.transform(input);
	}
</script>

<button onclick={() => process(data)}>
	{realValue.label}
</button>
