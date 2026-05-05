<script>
    let stateBase = 0;
    let derivedBase = 1;
    let propsBase = {};
    let effectFn = () => {};
    let inspectVal = "x";
    let bindableDefault = false;

    let s = $state(stateBase);
    let r = $state.raw(propsBase);
    let snap = $state.snapshot(propsBase);
    let d = $derived(derivedBase);
    let db = $derived.by(() => derivedBase);
    let p = $props();
    let pid = $props.id();
    let t = $effect.tracking();
    $effect(effectFn);
    $effect.pre(effectFn);
    $inspect(inspectVal);
    let b = $bindable(bindableDefault);
</script>
