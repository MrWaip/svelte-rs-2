<script lang="ts">
    import {debounce} from 'es-toolkit';

    type StateRune = {
        value: boolean,
    }

    let value = $state<StateRune>('long string value');
</script>