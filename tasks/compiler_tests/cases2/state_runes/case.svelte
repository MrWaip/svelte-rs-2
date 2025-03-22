<script lang="ts">
    import {debounce} from 'es-toolkit';

    type StateRune = {
        value: boolean,
    }

    let value = $state<StateRune>('long string value');
    let value2 = $state(undefined);
    let value3 = $state();
    let value4 = $state({});
    let value5 = $state(value1);
    let value6 = $state(null);
    let value7 = $state(() => {});
</script>