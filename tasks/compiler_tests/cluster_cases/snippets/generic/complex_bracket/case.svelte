<script lang="ts"></script>

{#snippet complex_generic<T extends { bracket: "<" } | "<" | Set<"<>">>(val: T)}
	{val}
{/snippet}
