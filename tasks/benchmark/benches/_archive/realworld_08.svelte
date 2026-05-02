<script lang="ts">
	import { i18n } from '@papper-ppr-foundation/i18n';
	import { trackMount } from '@papper-ppr-foundation/tracker';
	import { IcMBellFilled, IcMVisibilityFilled } from '@papper-ppr-ds/icons';
	import { isNil } from 'es-toolkit';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	import Button from '$components/Button/Button.svelte';
	import Cell from '$components/CellV2/Cell.svelte';
	import CellBox from '$components/CellV2/CellBox.svelte';
	import CellContainer from '$components/CellV2/CellContainer.svelte';
	import CellContent from '$components/CellV2/CellContent.svelte';
	import ChipRadioGroup from '$components/Chip/ChipRadioGroup.svelte';
	import Header from '$components/Header/Header.svelte';
	import Icon from '$components/Icon/Icon.svelte';
	import NavBackButton from '$components/NavButtons/NavBackButton.svelte';
	import MoneyInput from '$components/MoneyInput/MoneyInput.svelte';
	import Toggle from '$components/Toggle/Toggle.svelte';
	import {
		LIGHT_TABLE_ID_PARAM,
		LIGHT_TABLE_MEMBER_ID_PARAM,
		LIGHT_TABLES_MEMBERS,
	} from '$constants/routes';
	import { apiClient } from '$gatewayService';
	import {
		JointLimitType,
		JointLimitTypeInput,
		JointSettingsHistory,
		JointSettingsHistoryInput,
		JointSettingsNotification,
		JointSettingsNotificationInput,
	} from '$gatewayService/__queryTypes';
	import { eventBus } from '$lib/eventBus';
	import { router } from '$lib/router';

	import Skeleton from './elements/Skeleton.svelte';
	import { tracker } from './tracker';

	let tableId = $state<string | null>(null);
	let memberId = $state<string | null>(null);
	let relationId = $state<string | null>(null);
	let loading = $state({ reset: false, set: false, limit: false });
	let error = $state('');

	let limit = $state<{ value: number | undefined; type: JointLimitTypeInput }>({
		value: undefined,
		type: JointLimitTypeInput.TYPE_DAILY,
	});

	let historySetting = $state(false);
	let notificationSetting = $state(false);

	onMount(async () => {
		loading.limit = true;
		const searchParams = new URLSearchParams(window.location.search);

		tableId = searchParams.get(LIGHT_TABLE_ID_PARAM)!;
		memberId = searchParams.get(LIGHT_TABLE_MEMBER_ID_PARAM)!;

		const result = await apiClient.lightTable_memberLimits({
			memberId: memberId!,
			tableId: tableId!,
		});

		if (result.isFail()) {
			return eventBus.snackbarV2.alert(
				i18n(
					'apps.mflightTableLimits.snackbar.loadError',
					'Lorem ipsum dolor sit amet consectetur adipiscing'
				)
			);
		}

		const loadedLimit = result.value?.limit;

		relationId = result.value?.relationId ?? null;
		historySetting = result.value?.historySettingsValue === JointSettingsHistory.HISTORY_OWN;
		notificationSetting =
			result.value?.notificationSettingsValue ===
			JointSettingsNotification.NOTIFICATION_ENABLED;

		if (loadedLimit?.monthly?.isIndividual) {
			limit.type = JointLimitTypeInput.TYPE_MONTHLY;
			limit.value = loadedLimit.monthly.value;
		} else if (loadedLimit?.daily?.isIndividual) {
			limit.type = JointLimitTypeInput.TYPE_DAILY;
			limit.value = loadedLimit.daily.value;
		}

		loading.limit = false;
	});

	function validateLimits(): boolean {
		let result = true;

		if (isNil(limit.value)) {
			error = i18n(
				'apps.mflightTableLimits.validation.required',
				'Lorem ipsum dolor sit amet consectetur'
			);
			result = false;
		}

		return result;
	}

	function clearError() {
		error = '';
	}

	async function changeSettings() {
		await apiClient.memberLightTable_change.call({
			input: {
				tableID: tableId!,
				relationID: relationId!,
				historySettingsValue: historySetting
					? JointSettingsHistoryInput.HISTORY_OWN
					: JointSettingsHistoryInput.HISTORY_ALL,
				notificationSettingsValue: notificationSetting
					? JointSettingsNotificationInput.NOTIFICATION_ENABLED
					: JointSettingsNotificationInput.NOTIFICATION_DISABLED,
			},
		});
	}

	async function handleHistoryToggle() {
		historySetting = !historySetting;
		await changeSettings();
	}

	async function handleNotificationToggle() {
		notificationSetting = !notificationSetting;
		await changeSettings();
	}

	async function setLimits() {
		try {
			if (!validateLimits()) return;

			loading.set = true;
			const limitsResult = await apiClient.lightTable_setMemberLimits({
				input: {
					tableID: tableId!,
					relationID: relationId!,
					limitType: limit.type,
					limitValue: limit.value!,
				},
			});

			if (limitsResult.isFail()) {
				return eventBus.snackbarV2.alert(
					i18n('apps.mflightTableLimits.snackbar.setError', 'Lorem ipsum dolor sit amet consectetur')
				);
			}

			router.goto(
				LIGHT_TABLES_MEMBERS({ tableId: tableId!, relationId: relationId! }).toString(),
				{ replaceState: true }
			);
			eventBus.snackbarV2.success(
				i18n('apps.mflightTableLimits.snackbar.setSuccess', 'Lorem ipsum dolor')
			);
		} finally {
			loading.set = false;
		}
	}

	async function resetLimits() {
		try {
			loading.reset = true;
			const result = await apiClient.lightTable_resetMemberLimits({
				input: {
					tableID: tableId!,
					relationID: relationId!,
				},
			});

			if (result.isFail()) {
				return eventBus.snackbarV2.alert(
					i18n('apps.mflightTableLimits.snackbar.resetError', 'Lorem ipsum dolor sit amet consectetur')
				);
			}

			router.goto(
				LIGHT_TABLES_MEMBERS({ tableId: tableId!, relationId: relationId! }).toString(),
				{ replaceState: true }
			);
			eventBus.snackbarV2.success(
				i18n('apps.mflightTableLimits.snackbar.resetSuccess', 'Lorem ipsum dolor')
			);
		} finally {
			loading.reset = false;
		}
	}

	trackMount(() => tracker.view.page());
</script>

<Header>
	{#snippet left()}
		<NavBackButton track={tracker.click.back()} />
	{/snippet}
</Header>

{#if !loading.limit}
	<div class="container" in:fade={{ duration: 700 }}>
		<div ppr-typo="tsHeadline600Large">
			{i18n('apps.mflightTableLimits.title', 'Lorem ipsum dolor')}
		</div>

		<div class="subtitle" ppr-typo="tsBody500Medium">
			{i18n(
				'apps.mflightTableLimits.subtitle',
				'Lorem ipsum dolor sit amet — consectetur adipiscing, elit sed eiusmod'
			)}
		</div>

		<div class="chips">
			<ChipRadioGroup
				track={tracker.click.dailyLimitChip()}
				size="600"
				title={i18n('apps.mflightTableLimits.chip.daily', 'Lorem')}
				value={JointLimitType.TYPE_DAILY}
				bind:group={limit.type}
			/>
			<ChipRadioGroup
				track={tracker.click.monthlyLimitChip()}
				size="600"
				title={i18n('apps.mflightTableLimits.chip.monthly', 'Ipsum')}
				value={JointLimitType.TYPE_MONTHLY}
				bind:group={limit.type}
			/>
		</div>

		<div class="input">
			<MoneyInput
				bind:cents={limit.value}
				dataTestid="input-limit-value"
				labelPosition="inside"
				status={error !== '' ? 'error' : undefined}
				caption={error}
				label={i18n('apps.mflightTableLimits.input.amount.label', 'Lorem')}
				onChange={clearError}
				onClear={clearError}
			/>
		</div>

		<div class="info">
			<CellContainer yPadding={18} xPadding={16} yGap={14} xGap={10}>
				<Cell dataTestid="view-operations" track={tracker.click.historyCell()}>
					<CellBox>
						<Icon icon={IcMVisibilityFilled} color="graphicTertiary" size="500" shape="none" />
					</CellBox>

					<CellBox>
						<CellContent
							titleNumberOfLines={2}
							title={i18n(
								'apps.mflightTableLimits.cell.history.title',
								'Lorem ipsum dolor sit amet consectetur adipi'
							)}
						/>
					</CellBox>

					<CellBox>
						<Toggle checked={historySetting} onchange={handleHistoryToggle} />
					</CellBox>
				</Cell>

				<Cell track={tracker.click.notificationCell()}>
					<CellBox>
						<Icon icon={IcMBellFilled} color="graphicTertiary" size="500" shape="none" />
					</CellBox>

					<CellBox>
						<CellContent
							titleNumberOfLines={2}
							title={i18n(
								'apps.mflightTableLimits.cell.notification.title',
								'Lorem ipsum dolor sit amet consectetur adipiscing'
							)}
							subtitle={i18n('apps.mflightTableLimits.cell.notification.subtitle', 'Lorem')}
						/>
					</CellBox>

					<CellBox>
						<Toggle checked={notificationSetting} onchange={handleNotificationToggle} />
					</CellBox>
				</Cell>
			</CellContainer>
		</div>

		<div class="controls">
			<Button
				size="600"
				loading={loading.reset}
				onclick={resetLimits}
				fullWidth
				color="actionSecondary"
				track={tracker.click.reset()}
			>
				{i18n('apps.mflightTableLimits.button.reset', 'Lorem')}
			</Button>

			<Button
				size="600"
				loading={loading.set}
				onclick={setLimits}
				fullWidth
				color="actionPrimary"
				track={tracker.click.set()}
			>
				{i18n('apps.mflightTableLimits.button.set', 'Ipsum')}
			</Button>
		</div>
	</div>
{:else}
	<div class="container">
		<Skeleton />
	</div>
{/if}

<style>
	.container {
		display: flex;
		min-height: 34rem;
		flex: 1 1 auto;
		flex-direction: column;
		padding: 1rem 2rem 2rem;
	}

	.subtitle {
		margin-top: 0.5rem;
		color: var(--textSecondary);
	}

	.chips {
		display: flex;
		margin-top: 1.5rem;
		gap: 0.5rem;
	}

	.input {
		margin-top: 1rem;
	}

	.info {
		margin-top: 1.5rem;
	}

	.controls {
		display: grid;
		margin-top: auto;
		gap: 0.75rem;
		/* stylelint-disable-next-line plugin/no-unsupported-browser-features */
		grid-auto-columns: minmax(0, 1fr);
		grid-auto-flow: column;
	}

	@media (--mobile) {
		.container {
			padding: 1rem;
		}

		.controls {
			gap: 0.5rem;
		}
	}
</style>
