<script lang="ts">
    import { isNotNil } from 'es-toolkit';

    import TransferFormLoading from '$apps/MFInternationalTransfersByPhone/views/TransferFormLoading.svelte';

    import { contextifiedStore } from '../store/RootStore.svelte';
    import { View } from '../store/views';
    import TransferFormLoaded from './TransferFormLoaded.svelte';

    const rootStore = contextifiedStore.getStore();
    const { payoffLazy } = rootStore.navigation.useViewPayload(View.TransferForm);
</script>

{#if isNotNil(payoffLazy.data)}
    <TransferFormLoaded bind:payoffStore={payoffLazy.data} />
{:else}
    <TransferFormLoading />
{/if}
