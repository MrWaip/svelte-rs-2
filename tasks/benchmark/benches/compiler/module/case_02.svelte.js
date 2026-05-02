import { i18n } from "@papper-ppr-foundation/i18n";
import { papperTicketDeeplink } from "@papper-ppr-foundation/links";
import { AlphaValidator, DefaultLimitStrategy } from "$apps/MFAlpha/limitManager.svelte";
import { URL_MAIN } from "$constants/routes";
import { apiClient } from "$gatewayService";
import { ProductTypeA, AlphaTransferDisplayAction, Card2TableTransferFromInput as TransferFrom } from "$gatewayService/__queryTypes";
import { withPoll } from "$helpers/poll/index.svelte";
import { contextifyStore } from "$lib/contexify";
import { ctx } from "$lib/context";
import { eventBus } from "$lib/eventBus";
import { lazy } from "$lib/lazyRune/lazy.svelte";
import { router } from "$lib/router";
import { CommissionCalculator } from "$shared/apps/MFAlpha/commissionCalculator.svelte";
import { getDefaultError, mapErrorsByDisplayAction } from "$shared/apps/MFAlpha/mapper";
import { alphaNavigation } from "$shared/navigation/c2a";
import { openSuccessC2ACurtain } from "./curtains/successC2ACurtain";
import { mapState } from "./mappers";
function makeStore(contract) {
	const commissionCalculator = new CommissionCalculator();
	let step = $state("form");
	let selectedTreeCard = $state();
	let error = $state();
	const state = lazy(null, async () => {
		const incomingUrlParams = alphaNavigation.extractIncomingUrlParam(window.location.search);
		const result = await apiClient.endpointA_init.call({ features: ["_"] });
		if (result.isFail()) return null;
		const data = mapState(result.value, incomingUrlParams.tableNumber);
		selectedTreeCard = data.treeCards?.[0];
		return data;
	});
	const endpointBTableId = $derived.by(() => {
		const currentTable = state.data?.currentTable;
		if (currentTable?.type === ProductTypeA.ACCOUNT_TYPE_A || currentTable?.type === ProductTypeA.ACCOUNT_TYPE_B || currentTable?.type === ProductTypeA.ACCOUNT_TYPE_C || currentTable?.type === ProductTypeA.ACCOUNT_TYPE_D) {
			return currentTable.number;
		}
	});
	function next(inStep) {
		step = inStep;
	}
	function tryAgain() {
		next("form");
	}
	const limitStrategy = contract.v1?.limitStrategy ?? new DefaultLimitStrategy(() => ({
		maxBalance: state.data?.maxBalanceLimit,
		spentBalance: state.data?.spentBalanceLimit
	}));
	const validator = new AlphaValidator(limitStrategy);
	async function createDeposit(payload) {
		const isWebview = ctx.deviceInfo.isWebview;
		const currentTable = state.data?.currentTable;
		const commission = commissionCalculator.commission;
		const returnUrl = alphaNavigation.buildReturnUrl(isWebview);
		if (!currentTable) {
			eventBus.snackbarV2.alert(i18n("mfalpha.createDeposit.error.currentTableNull", "lorem-lorem lorem lorem lorem, lorem lorem"));
			throw new Error("C2A: currentTable is null");
		}
		if (!validator.validate(payload.amount, commission)) {
			return;
		}
		eventBus.progressStartLoader();
		const result = await apiClient.endpointA_createV2.call({ input: {
			amount: payload.amount,
			failUrl: returnUrl,
			successUrl: returnUrl,
			transferFrom: contract.v1?.topDisclaimer ? TransferFrom.PPR_CHECKOUT : TransferFrom.TREE_CABINET,
			commission: commission.value,
			replenishmentTableNumber: currentTable.number,
			cardToken: selectedTreeCard?.token
		} });
		eventBus.progressStop();
		if (result.isFail()) {
			return eventBus.snackbarV2.alert(i18n("mfalpha.createDeposit.error.operationFail", "lorem lorem lorem lorem"));
		}
		const transfer = result.value.transfer;
		error = mapErrorsByDisplayAction(result.value.displayAction, tryAgain);
		if (error) return next("fail");
		let targetUrl = transfer?.ticketFormUrl;
		if (targetUrl) {
			if (isWebview) {
				targetUrl = papperTicketDeeplink({ ticketUrl: targetUrl });
			}
			return window.location.replace(targetUrl);
		}
		if (!transfer?.transferId) {
			return eventBus.snackbarV2.alert(i18n("mfalpha.createDeposit.error.operationFail", "lorem lorem lorem lorem"));
		}
		const redirectUrl = alphaNavigation.setTransactionNumberInUrl(transfer?.transferId);
		router.goto(redirectUrl, { replaceState: true });
	}
	const pollStatus = withPoll(apiClient.endpointA_statusV2, {
		validate: (result) => {
			if (result.isFail()) return false;
			if (result.value.displayAction) {
				if (result.value.displayAction === AlphaTransferDisplayAction.ALPHA_DISPLAY_ACTION_PROCESSING) return false;
				return true;
			}
			return false;
		},
		interval: 500
	});
	function init() {
		commissionCalculator.calculate(contract.v1?.initialAmount ?? 0);
	}
	async function notifyTransferSuccess({ transferAmt, cardMask }) {
		await openSuccessC2ACurtain({
			amount: transferAmt,
			pan: cardMask?.slice(-4) || ""
		});
	}
	async function verifyTransferResult(params) {
		const { redirectUrl, redirectWithSuccessNotification, transactionNumber } = params;
		if (!transactionNumber) return;
		eventBus.progressStart("");
		const pollingResult = await pollStatus({ transferId: transactionNumber });
		eventBus.progressStop();
		if (pollingResult.isFail()) {
			alphaNavigation.cleanUpTransferId();
			eventBus.snackbarV2.alert(i18n("mfalpha.verifyTransfer.error.transferFail", "lorem lorem lorem lorem"));
			return;
		}
		const result = pollingResult.value;
		const transfer = result.value?.transfer;
		if (result.isFail()) {
			alphaNavigation.cleanUpTransferId();
			eventBus.snackbarV2.alert(i18n("mfalpha.verifyTransfer.error.transferFail", "lorem lorem lorem lorem"));
			return;
		}
		if (result.value.displayAction === AlphaTransferDisplayAction.ALPHA_DISPLAY_ACTION_SUCCESS && transfer) {
			if (contract.v1?.onSuccess) {
				return contract.v1.onSuccess();
			}
			if (redirectUrl) {
				if (redirectWithSuccessNotification) {
					notifyTransferSuccess({
						transferAmt: transfer.amount.cents,
						cardMask: transfer.cardMask
					});
				}
				return router.goto(redirectUrl, { replaceState: true });
			}
			router.goto(URL_MAIN.toString(), { replaceState: true });
			notifyTransferSuccess({
				transferAmt: transfer.amount.cents,
				cardMask: transfer.cardMask
			});
			return;
		}
		error = mapErrorsByDisplayAction(result.value.displayAction, tryAgain);
		alphaNavigation.cleanUpTransferId();
		if (contract.v1?.onFail) {
			return contract.v1.onFail();
		}
		next("fail");
	}
	function selectTreeCard(selectedToken) {
		selectedTreeCard = state.data?.treeCards.find(({ token }) => token == selectedToken);
	}
	return {
		get state() {
			return state;
		},
		get error() {
			return error;
		},
		get defaultError() {
			return getDefaultError(tryAgain);
		},
		get step() {
			return step;
		},
		get endpointBTableId() {
			return endpointBTableId;
		},
		get selectedTreeCardToken() {
			return selectedTreeCard?.token;
		},
		next,
		init,
		verifyTransferResult,
		selectTreeCard,
		commissionCalculator,
		createDeposit,
		limitValidator: validator
	};
}
export const { getStore, initStore } = contextifyStore("C2A Store", makeStore);
