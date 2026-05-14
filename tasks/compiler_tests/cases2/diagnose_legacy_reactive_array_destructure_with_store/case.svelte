<script>
    export let onSubmit;
    export let pair;

    function withoutConcurrent(fn) {
        return [fn, { subscribe: () => () => {} }];
    }

    $: [run, isLoading] = withoutConcurrent(onSubmit);
    $: [[a, b], [c, d]] = pair;

    function go() {
        run();
    }
</script>

<button on:click={go}>{$isLoading ? 'wait' : 'go'}-{a}-{b}-{c}-{d}</button>
