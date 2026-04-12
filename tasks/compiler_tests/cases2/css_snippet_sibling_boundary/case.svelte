<style>
    .before + .after { color: red; }
</style>

<span class="before">before</span>

{#snippet pair()}
    <div class="after">after</div>
{/snippet}

{@render pair()}

<div>other</div>
