<style>
    .card {
        & .title { color: red; }
    }
    .panel {
        .label { color: blue; }
    }
</style>

<div class="card">
    <h2 class="title">inside</h2>
</div>

<section class="panel">
    <h3 class="label">implicit</h3>
</section>

<h2 class="title">outside</h2>
<h3 class="label">outside implicit</h3>
