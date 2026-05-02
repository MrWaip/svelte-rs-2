import { i18n } from '@papper-ppr-foundation/i18n';
import { v4 as uuidv4 } from 'uuid';

import { GOOGLE_CARD_TICKET } from '$constants/routes';
import { apiClient } from '$gatewayService';
import {
	ProductTypeA,
	type ClientRepayRequisitesV1Input,
	GroupOperationTypeV3Input,
	OnlineControlResponseDecision,
	AGENTResult,
	type TransferBetweenTablesRequisitesV1Input,
} from '$gatewayService/__queryTypes';
import { showBottomSheetCustom } from '$helpers/CBottom/showBottomSheetCustom';
import { moneyFormatterFactory } from '$helpers/money';
import { withPoll } from '$helpers/poll/index.svelte';
import { tableService } from '$lib/tableService';
import { clientLogger } from '$lib/clientLogger';
import { contextifyStore } from '$lib/contexify';
import { AppEvents, eventBus } from '$lib/eventBus';
import { limitService } from '$lib/limitService';
import { tryShowPushNotifications } from '$lib/pushNotification';
import { router } from '$lib/router';
import { withLoading } from '$lib/synchronization/withLoading.svelte';
import { sendTrackingEvent } from '$lib/tracker';
import { userDataSigner } from '$lib/userDataSigner';
import { getAmountError as getEndpointBAmountError } from '$shared/apps/MFBeta/helpers/getAmountError';
import { mapMe2meFailTransferStatus } from '$shared/apps/MFBeta/mapper/mapMe2meFailTransferStatus';

import { openEnableAgentCurtain } from '../../../curtains/enableAgentCurtain';
import { openTransferFailByArrest } from '../../../curtains/transferFailByArrest';
import { openFailTransferCurtain } from '../curtains/failTransfer';
import SuccessTransferModal from '../elements/SuccessTransferModal/SuccessTransferModal.svelte';
import { getGoogleCardProductsDetails } from '../helpers/getGoogleCardProductDetails';
import { redirect } from '../helpers/redirect';
import { tracker } from '../tracker';
import {
	type Table,
	type Tree,
	type GroupOperationInfo,
	EndpointBTransferResult,
	type State,
	Steps,
} from '../types';

export const createStore = (initState: State) => {
	const state = $state<State>(initState);

	const fmt = moneyFormatterFactory();

	const setSelectedTable = (direction: 'from' | 'to', key: string) => {
		if (direction === 'from') {
			const targets = state.sourcesKeyToTargets.get(key) ?? [];
			const currentSelectedToKey = state.selectedSources.to.key;

			/** lorem lorem lorem from lorem, lorem lorem to lorem lorem lorem */
			if (!targets.includes(currentSelectedToKey)) {
				state.selectedSources.to = state.sourcesByKey.get(targets[0] ?? '') as Table;
			}

			state.sourcesTo = targets
				.map((key) => state.sourcesByKey.get(key))
				.filter((item) => item?.type === 'table');

			state.selectedSources.from = state.sourcesByKey.get(key)!;

			if (state.cents) validateAmount(state.cents);
		} else {
			state.selectedSources.to = state.sourcesByKey.get(key) as Table;
		}
	};

	const clearError = () => (state.error = undefined);

	const resetTransferState = () => {
		state.endpointB.transferResult = undefined;
		state.idempotencyKey = uuidv4();
	};

	const validateAmount = (amount: number | undefined): boolean => {
		clearError();

		const from = state.selectedSources.from;
		const to = state.selectedSources.to;

		if (!amount) {
			state.error = i18n('mfTransferBetween.amount.error.empty', 'lorem lorem');
			return false;
		}

		if (state.minTransferAmount && amount < state.minTransferAmount) {
			state.error = i18n(
				'mfTransferBetween.amount.error.minTransferAmount',
				'lorem lorem lorem lorem lorem {minAmount}',
				{ minAmount: fmt(state.minTransferAmount) }
			);
			return false;
		}

		if (from.type === 'tree') {
			const { minLimit, maxLimit } = state.endpointB.limits;
			const error = getEndpointBAmountError(amount, minLimit, maxLimit);

			if (error) {
				state.error = error;
				return false;
			}
		} else {
			if (from.balance < amount) {
				state.error = i18n('mfTransferBetween.amount.error.notEnough', 'lorem lorem');
				return false;
			}

			if (tableService.isGoogleTable(from.productTypeV3)) {
				const minLimit = limitService.getMinTransferGoogleAmount();

				if (amount < minLimit) {
					state.error = i18n(
						'mfTransferBetween.amount.error.googleMinTransfer',
						'lorem lorem lorem {minLimit}',
						{ minLimit: fmt(minLimit) }
					);
					return false;
				}
			}
		}

		if (tableService.isGoogleTable(to.productTypeV3)) {
			const fullDebt = getGoogleCardProductsDetails(to)?.fullDebt ?? 0;

			if (fullDebt > 0 && amount > fullDebt) {
				state.error = i18n(
					'mfTransferBetween.amount.error.googleMaxTransfer',
					'lorem lorem lorem {maxLimit}',
					{
						maxLimit: fmt(fullDebt),
					}
				);
				return false;
			}
		}

		return true;
	};

	const pollOperation = withPoll(apiClient.endpointG_operation, {
		validate: (result) => {
			return Boolean(result.isOk() && result.value.operationId);
		},
		interval: 200,
		maxAttempts: 3,
	});

	const showSuccessTransferCurtain = async (
		from: Table,
		to: Table,
		amount: number,
		operationInfo?: GroupOperationInfo
	) => {
		if (state.isPapperTicket) return;

		const operationPolling = operationInfo
			? await pollOperation({
					groupID: operationInfo.groupOperationID,
					groupOperationType:
						operationInfo.groupOperationType as unknown as GroupOperationTypeV3Input,
				})
			: undefined;

		const operation = operationPolling?.value?.value;

		try {
			const data = { from, to, amount, isPapperTicket: false, operation };
			const type = await state.onSuccessTransferNotification(data);

			if (type === 'custom') return;
		} catch (error) {
			const err = error as Error;
			clientLogger.warn(err, 'MFGamma: onSuccessTransferNotification throw error');
		}

		showBottomSheetCustom({
			component: SuccessTransferModal,
			props: {
				fromTableName: from.tableName,
				toTableName: to.tableName,
				amount,
				operationId: operation?.operationId,
				canTemplate: operation?.canTemplate,
				onDestroy: () => showEnablePushNotificationCurtain(to.productTypeV3),
			},
		});
	};

	const showFailTransferNotice = () => {
		eventBus.snackbarV2.alert(
			i18n('mfTransferBetween.transfer.error.message', 'lorem lorem lorem lorem'),
			i18n('mfTransferBetween.transfer.error.description', 'lorem, lorem lorem lorem')
		);
	};

	const showEnablePushNotificationCurtain = (toTableType: ProductTypeA) => {
		if (!state.features.enablePushNotifications || tableService.isGoogleTable(toTableType)) {
			return;
		}

		const type = tableService.isSavingsTable(toTableType) ? 'savingsTopUp' : 'endpointBTransfers';

		tryShowPushNotifications({ type });
	};

	const enableAgent = async () => {
		const result = await apiClient.clientRules_create.call({
			input: { allow: true, groupCode: 'agent' },
		});

		if (result.isOk()) {
			state.isAgentEnabled = true;

			eventBus.dispatchEvent({ name: AppEvents.ENABLE_AGENT, payload: null });
			eventBus.snackbarV2.success(
				i18n(
					'mfTransferBetween.enableAgent.snackbar.success.title',
					'lorem lorem{NBSP}lorem lorem lorem'
				)
			);
			sendTrackingEvent(tracker.view.agentEnableSuccessSnackbar());

			return true;
		}

		eventBus.snackbarV2.alert(
			i18n('mfTransferBetween.enableAgent.snackbar.fail.title', 'lorem lorem lorem lorem'),
			i18n('mfTransferBetween.enableAgent.snackbar.fail.subtitle', 'lorem lorem lorem')
		);
		sendTrackingEvent(tracker.view.agentEnableFailureSnackbar());

		return false;
	};

	const transferBetweenTables = async (
		from: Table,
		to: Table,
		cents: number,
		idempotencyKey: string
	) => {
		const data2sign: TransferBetweenTablesRequisitesV1Input = {
			amountCents: cents,
			amountCurrencyCode: '111',
			clientID: state.clientId,
			requestID: uuidv4(),
			toTableNumber: to.tableNumber,
			fromTableNumber: from.tableNumber,
		};

		const { requisitesSign, signToken } = await userDataSigner.sign(data2sign);

		const result = await apiClient.endpointG_transferV3.call({
			input: {
				idempotencyKey,
				requisitesSign,
				signToken,
				requisitesV1: data2sign,
			},
		});

		if (result.isFail() || !result.value) {
			showFailTransferNotice();
			return;
		}

		const { onlineControl, success, groupOperationInfo } = result.value;

		if (onlineControl?.decision === OnlineControlResponseDecision.CHALLENGE) {
			state.step = Steps.Challenge;
			state.challengeId = onlineControl.challenge?.id;
			return;
		}

		if (!success) {
			showFailTransferNotice();
			return;
		}

		await showSuccessTransferCurtain(from, to, cents, groupOperationInfo);

		if (state.onSuccessTransfer) {
			await state.onSuccessTransfer();
			return;
		}

		await redirect(state.urlParams);
	};

	const transferEndpointB = async (from: Tree, to: Table, cents: number, idempotencyKey: string) => {
		const result = await withPoll(
			() =>
				apiClient.endpointBTransfer.call({
					input: {
						treeId: from.id,
						tableToken: to.tableToken,
						amount: cents,
						operationID: idempotencyKey,
					},
				}),
			{
				interval: 500,
				maxAttempts: 20,
				validate: (result) => result.value !== AGENTResult.AGENT_OPERATION_TIMEOUT,
			}
		)();

		state.step = Steps.ME2ME_RESULT;

		if (result.isFail()) {
			state.endpointB.transferResult = EndpointBTransferResult.Fail;
			return;
		}

		switch (result.value.value) {
			case AGENTResult.OK_RESULT:
				state.endpointB.transferResult = EndpointBTransferResult.Success;
				break;
			case AGENTResult.GOOGLE_TRANSFER_TIMEOUT_IN_PROGRESS:
				state.endpointB.transferResult = EndpointBTransferResult.Pending;
				break;
			default:
				state.endpointB.transferResult = EndpointBTransferResult.Fail;
				state.endpointB.failStatus = mapMe2meFailTransferStatus(result.value.value);
		}
	};

	const transferToGoogleCard = async (
		from: Table,
		to: Table,
		cents: number,
		idempotencyKey: string
	) => {
		const data2sign: ClientRepayRequisitesV1Input = {
			amount: cents,
			clientID: state.clientId,
			idempotencyKey,
			debitTableNumber: from.tableNumber,
			limitTableNumber: to.tableNumber,
		};

		const { requisitesSign, signToken } = await userDataSigner.sign(data2sign);

		const result = await apiClient.endpointG_googleReticket.call({
			input: {
				signToken,
				requisitesV1: data2sign,
				requisitesSign,
			},
		});

		if (result.isFail() || !result.value) {
			showFailTransferNotice();
			return;
		}

		await showSuccessTransferCurtain(from, to, cents);

		if (state.onSuccessTransfer) {
			await state.onSuccessTransfer();
			return;
		}

		await redirect(state.urlParams);
	};

	const submit = withLoading(async () => {
		const from = state.selectedSources.from;
		const to = state.selectedSources.to;
		const cents = state.cents ?? 0;
		const idempotencyKey = state.idempotencyKey;
		const isPapperTicket = state.isPapperTicket;

		if (!from || !to) {
			clientLogger.warn(`transferBetween: selected sources not found: from:${from} -> to:${to}`);
			return;
		}

		if (from.type === 'tree' && !state.isAgentEnabled) {
			openEnableAgentCurtain({
				title: i18n(
					'mfTransferBetween.enableAgentCurtain.title',
					'lorem lorem,{NBSP}lorem lorem lorem{NBSP}lorem'
				),
				onEnable: enableAgent,
				trackClick: tracker.click.enableAgent(),
				trackShow: tracker.view.enableAgentCurtain(),
				trackClose: tracker.close.enableAgentCurtain(),
			});
			return;
		}

		if (from.type === 'table' && from.googleOverdueAmount) {
			return router.goto(GOOGLE_CARD_TICKET.toString());
		}

		if (from.type === 'table' && from.restriction?.isUnderArrest) {
			const availableBalance = from.restriction.availableBalance ?? 0;

			if (availableBalance < cents) {
				return openTransferFailByArrest({
					tableId: from.tableNumber,
					availableBalance,
					source: 'transfer-between-tables',
				});
			}
		}

		const isValid = validateAmount(cents);
		if (!isValid) return;

		const executeTransfer = async () => {
			if (from.type === 'tree') {
				await transferEndpointB(from, to, cents, idempotencyKey);
			} else if (tableService.isGoogleTable(to.productTypeV3)) {
				await transferToGoogleCard(from, to, cents, idempotencyKey);
			} else {
				await transferBetweenTables(from, to, cents, idempotencyKey);
			}
		};

		try {
			const data = { from, to, amount: cents, isPapperTicket };
			const nextAction = await state.onPrepareTransfer(data, executeTransfer);

			if (nextAction === 'pause') return;
		} catch (error) {
			const err = error as Error;
			clientLogger.warn(err, 'MFGamma: onPrepareTransfer throw error');
		}

		await executeTransfer();
	});

	const handleChallengeSuccess = () => {
		sendTrackingEvent(tracker.challenge.success());
		state.step = Steps.Form;
		submit.execute();
	};

	const handleChallengeFail = () => {
		sendTrackingEvent(tracker.challenge.fail());
		openFailTransferCurtain({
			onClose: () => {
				state.step = Steps.Form;
			},
		});
	};

	return {
		state,
		submit,
		setSelectedTable,
		clearError,
		resetTransferState,
		handleChallengeSuccess,
		handleChallengeFail,
	};
};

export type Store = ReturnType<typeof createStore>;

export const { getStore, initStore } = contextifyStore('TransferBetweenStore', createStore);
