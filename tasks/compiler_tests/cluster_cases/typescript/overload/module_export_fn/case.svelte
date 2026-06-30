<script module lang="ts">
	export function f(a: boolean): boolean;
	export function f(a: string): number;
	export function f(a: any): any { return a; }
</script>
<script lang="ts">
	let count = $state(0);
</script>
<button onclick={() => count++}>{count}</button>
