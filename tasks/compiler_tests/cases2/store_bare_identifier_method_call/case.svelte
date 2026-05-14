<script>
    import { timerStore } from './store';

    let timerId = $state();

    function clear() {
        if (timerId) timerStore.clearTimer(timerId);
    }

    function start() {
        timerId = timerStore.createTimer(60);
    }
</script>

<button onclick={start}>start</button>
<button onclick={clear}>clear</button>
{#if timerId && $timerStore[timerId]}active{/if}
