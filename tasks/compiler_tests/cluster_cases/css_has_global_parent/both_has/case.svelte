<x><y></y></x>

<style>
	:global(.foo) {
		:has(x) { color: green; }
		&:has(x) { color: green; }
	}
</style>
