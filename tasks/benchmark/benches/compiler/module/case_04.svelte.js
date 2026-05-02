import { i18n } from "@papper-ppr-foundation/i18n";
import { failResult } from "@papper-ppr-toolbox/result";
import { isNil } from "es-toolkit";
import { v4 as uuidv4 } from "uuid";
import { makeTicketTemplateStore } from "$components/TicketTemplate/store/ticketTemplateStore.svelte";
import { Error } from "$gatewayService/__queryTypes";
import { CardTranferHint, CardTransferType, GroupOperationTypeV2, FlowProcessFailedReason, TicketTemplateType } from "$gatewayService/__queryTypes";
import { apiClient } from "$gatewayService/generated/client";
import { clientLogger } from "$lib/clientLogger";
import { contextifyStore } from "$lib/contexify";
import { eventBus } from "$lib/eventBus";
import { router } from "$lib/router";
import { withLoading } from "$lib/synchronization/withLoading.svelte";
import { userDataSigner } from "$lib/userDataSigner";
import { openTransferFailByArrest } from "../../../curtains/transferFailByArrest";
import { openChallengeFailCurtain } from "../curtains/challengeFail";
import { getDisclaimerProps } from "../helpers/getCardInputDisclaimerProps";
import { mapInitErrorMessage, mapFlowStepToStatus, mapTicketToErrorMessage } from "../helpers/mappers";
import { convertToCardTransferTypeInput } from "../helpers/typeConverter";
import { getClientLimits } from "../helpers/userLimits";
import { validate } from "../helpers/validateAmount";
import { getIframeContext } from "../iframeModule/iframe.service";
function makeStore(initState) {
	const state = $state(initState);
	const iframe = getIframeContext();
	const iframeMethod = iframe.getIframeMethods();
	const ticketTemplateStore = makeTicketTemplateStore();
	const { setTemplateData } = ticketTemplateStore;
	const limits = $derived.by(() => {
		return getClientLimits({
			treeLimits: state.treeLimits,
			minAmount: state.recipientTransferInfo?.minAmount,
			transferType: state.recipientTransferInfo?.transferType
		});
	});
	const merchant = $derived.by(() => {
		if (!state.ticketSystem?.treeMerchant) {
			return undefined;
		}
		return {
			image: state.ticketSystem?.treeMerchant?.logoUrlLight ?? "",
			tree: state.ticketSystem?.treeMerchant?.name,
			name: state.recipientTransferInfo?.receiverName ?? ""
		};
	});
	const totalAmount = $derived.by(() => {
		if (state.commission.value && state.amount) {
			return state.commission.value + state.amount;
		}
		return state.amount;
	});
	const calculateCommission = withLoading(async () => {
		const newCommission = await loadCommission(state.amount);
		state.commission = newCommission;
		validateAmount();
	});
	if (state.templateData.templateId) {
		setTemplateData({
			id: state.templateData.templateId,
			name: state.templateData.templateName || "",
			treeId: "",
			amount: state.amount,
			tableNumber: state.currentTable.number,
			type: TicketTemplateType.TRANSFER_BY_CARD_NUMBER,
			minLimit: limits.singleCurrentOperation.min,
			maxLimit: limits.singleCurrentOperation.max
		});
	}
	if (state.mode === "Repeat") {
		calculateCommission.execute();
	}
	function updateCardNumberError(message) {
		iframeMethod.updateError(message);
	}
	function clearTicketSystemData() {
		state.ticketSystem = undefined;
		iframeMethod.resetTicketSystem();
	}
	function clearTransferData() {
		state.recipientTransferInfo = undefined;
		calculateCommission.execute();
		state.cardInputDisclaimer = undefined;
	}
	async function changeTicketSystemByBin(bin, isThemeDark) {
		const system = await iframeMethod.changeTicketSystemByBin(bin, isThemeDark);
		state.ticketSystem = system;
	}
	const setSelectedTable = (key) => {
		const table = state.tables.get(key);
		if (table) {
			state.currentTable = table;
			if (state.amount) validateAmount();
			return;
		}
		clientLogger.warn(`transferByCardNumber: Table with key ${key} not found`);
	};
	async function updateAmount(inAmount) {
		state.amount = inAmount;
		await calculateCommission.execute();
	}
	async function handleChallengeSuccess() {
		const challenge = state.challenge;
		if (challenge?.challengeFlow === "perform") {
			await fineTransfer.execute();
			return;
		} else if (challenge?.challengeFlow === "card") {
			const initData = state.recipientTransferInfo;
			if (!initData?.clientCardId || !initData?.maskedPan) return;
			await setupCardTransfer(initData?.clientCardId, initData.maskedPan);
			return state.step = "form";
		}
		return state.step = "fail";
	}
	const handleChallengeFail = () => {
		openChallengeFailCurtain({ onClose: () => {
			router.goBack();
		} });
	};
	async function setupCardTransfer(clientCardId, inMaskedPan) {
		const fail = (message) => {
			updateCardNumberError(message);
			state.commission = {
				status: "notcalculated",
				value: 0
			};
			return failResult("any error");
		};
		const result = await apiClient.endpointD_initDomesticTransfer.call({ input: {
			clientCardId,
			idempotencyKey: state.transferKey
		} });
		if (result.isFail()) {
			clearTransferData();
			const errorMessage = result.error.message;
			if (errorMessage.includes(Error.CARD_TRANSFER_ERROR) || errorMessage.includes(Error.TEMPORARILY_UNABLE_TO_TRANSFER_TO_SPECIFIED_PPR_CARD)) {
				state.cardInputDisclaimer = getDisclaimerProps("AGENT_TRANSFER");
			}
			if (errorMessage.includes(Error.UNABLE_TO_TRANSFER_TO_SELF_PPR_CARD)) {
				state.cardInputDisclaimer = getDisclaimerProps("SELF_TRANSFER");
			}
			const errorText = mapInitErrorMessage(errorMessage);
			if (errorText) {
				return fail(errorText);
			}
			return failResult("api error");
		}
		const localTransferType = result.value?.transferType;
		if (localTransferType === CardTransferType.UNAVAILABLE_TRANSFER) {
			state.cardInputDisclaimer = getDisclaimerProps("ABROAD_TRANSFER");
			return fail(i18n("endpointD.initTransfer.error.unavailableTransfer", "lorem lorem lorem lorem lorem lorem"));
		}
		if (!result.value.payoffId) {
			return fail(i18n("endpointD.initTransfer.error.payoffId.isBlank", "lorem lorem lorem lorem lorem lorem lorem"));
		}
		state.recipientTransferInfo = {
			payoffId: result.value.payoffId,
			maskedPan: inMaskedPan,
			transferType: localTransferType,
			minAmount: result.value.minAmount,
			receiverName: result.value.receiverFullName,
			clientCardId
		};
		calculateCommission.execute();
		if (result.value.hints.includes(CardTranferHint.HINT_TRANSFER_WILL_BE_SENT_TO_MAIN_RECIPIENT_TABLE)) {
			iframeMethod.setCaption(i18n("endpointD.cardNumberIframe.transferHint", "lorem lorem lorem {tableName} lorem", { tableName: state.currentTable.name }));
		}
		if (result.value.challenge) {
			state.challenge = {
				challengeId: result.value.challenge.id,
				challengeFlow: "card"
			};
			state.step = "challenge";
			return;
		}
	}
	async function loadCommission(amount) {
		if (isNil(amount) || !state.recipientTransferInfo?.payoffId || !state.recipientTransferInfo?.transferType) {
			return {
				status: "notcalculated",
				value: 0
			};
		}
		const result = await apiClient.endpointD_prepareDomesticTransfer.call({ input: {
			amount: {
				currencyCode: "111",
				cents: amount
			},
			payoffId: state.recipientTransferInfo?.payoffId,
			transferType: convertToCardTransferTypeInput(state.recipientTransferInfo?.transferType),
			tableToken: state.currentTable.token
		} });
		if (result.isFail()) {
			return {
				status: "fail",
				value: 0
			};
		}
		if (state.recipientTransferInfo?.transferType === CardTransferType.INTERNAL_TRANSFER) {
			return {
				status: "ok",
				value: 0
			};
		}
		return {
			status: "ok",
			value: result?.value?.clientCommission?.sum ?? 0
		};
	}
	function validateAmount() {
		const result = validate({
			limits,
			total: state.commission.value + state.amount,
			table: state.currentTable,
			amount: state.amount,
			commission: state.commission
		});
		state.amountError = result.errorText;
		return result;
	}
	const fineTransfer = withLoading(async () => {
		const result = await performTransfer();
		if (result === "challenge") {
			state.step = "challenge";
		}
		if (result === "next") {
			state.step = "final";
		}
	});
	async function getOperationOrDefault(groupOperationId, transferType) {
		const result = await apiClient.endpointD_operationSummary.call({
			operationType: transferType === CardTransferType.INTERNAL_TRANSFER ? GroupOperationTypeV2.C_2_C_BY_CARD_NUMBER_INTERNAL_OUTGOING : GroupOperationTypeV2.TRANSFER_BY_CARD_NUMBER_OUTGOING,
			groupOperationId
		});
		const operation = result.value;
		return {
			transferId: operation?.id ?? "",
			type: operation?.groupOperationType ?? GroupOperationTypeV2.TRANSFER_BY_CARD_NUMBER_OUTGOING,
			operationId: operation?.operationId ?? "",
			merchant: operation?.merchantName ?? "",
			canRepeat: operation?.canRepeat ?? false,
			canTemplate: operation?.canTemplate ?? false,
			commission: operation?.commission,
			logo: {
				light: operation?.image?.default,
				dark: operation?.image?.dark
			}
		};
	}
	async function performTransferSigned(amountCents, payoffId, transferType) {
		const data2Sign = {
			amountCents,
			amountCurrencyCode: "111",
			clientID: state.clientId,
			requestID: uuidv4(),
			clientCardId: state.recipientTransferInfo?.clientCardId || ""
		};
		const { requisitesSign, signToken } = await userDataSigner.sign(data2Sign);
		return await apiClient.endpointD_performTransferV3.call({ input: {
			signToken,
			requisitesSign,
			payoffId,
			transferType: convertToCardTransferTypeInput(transferType),
			requisitesV2: data2Sign,
			tableToken: state.currentTable.token
		} });
	}
	async function performTransfer() {
		const sum = state.amount;
		const commission = state.commission;
		const payoffId = state.recipientTransferInfo?.payoffId;
		const table = state.currentTable;
		const transferType = state.recipientTransferInfo?.transferType;
		if (!payoffId) {
			updateCardNumberError(i18n("endpointD.fineTransfer.error.payoffId.isBlank", "lorem lorem lorem lorem"));
		}
		if (isNil(sum)) {
			state.amountError = i18n("endpointD.fineTransfer.error.sum.isBlank", "lorem lorem lorem lorem");
		}
		if (isNil(sum) || !payoffId || !transferType) {
			return "validate";
		}
		switch (validateAmount().validation) {
			case "arrest":
				openTransferFailByArrest({
					availableBalance: table.restriction?.availableBalance,
					source: "transfer-by-card-number"
				});
				return "validate";
			case "other": return "validate";
			case "ok":
		}
		const isCommissionValid = commission.status === "ok";
		if (!isCommissionValid) {
			if (!calculateCommission.isLoading) {
				await calculateCommission.execute();
			}
			eventBus.snackbarV2.info(i18n("endpointD.formView.snackbar.commissionLoading.title", "lorem lorem"), i18n("endpointD.formView.snackbar.commissionLoading.description", "lorem lorem lorem lorem"));
			return "validate";
		}
		const fineResult = await performTransferSigned(sum, payoffId, transferType);
		if (fineResult.isFail()) {
			eventBus.snackbarV2.alert(i18n("endpointD.fineTransfer.error.transferFail", "lorem lorem lorem lorem"));
			return "error";
		}
		if (fineResult.value.payoff.challenge) {
			state.challenge = {
				challengeId: fineResult.value.payoff.challenge.id,
				challengeFlow: "perform"
			};
			return "challenge";
		}
		const payoff = fineResult.value.payoff;
		const needTransferApprove = payoff.failedReason === FlowProcessFailedReason.REASON_ONLINE_CONTROL_DECLINED_APPROVE_REQUIRED;
		const operation = await getOperationOrDefault(payoff.id, transferType);
		let approverId = "";
		if (needTransferApprove) {
			approverId = (await apiClient.ticketApproversLinks.call()).value ?? "";
		}
		state.operationSummaryData = {
			reason: mapTicketToErrorMessage(payoff.step, payoff.failedReason),
			merchant: operation.merchant,
			amount: payoff.amount.sum,
			commission: operation.commission,
			title: needTransferApprove && approverId ? i18n("endpointD.fineTransfer.needTransferApprove", "lorem lorem") : undefined,
			status: mapFlowStepToStatus(payoff.step),
			approverId,
			cardNumber: state.recipientTransferInfo?.maskedPan,
			operationId: operation.operationId,
			ticketSystemLogo: {
				dark: state.ticketSystem?.ticketSystemInfo?.darkCdnUrl,
				light: state.ticketSystem?.ticketSystemInfo?.lightCdnUrl
			},
			logo: operation.logo,
			canRepeat: operation.canRepeat,
			canTemplate: state.mode === "Repeat" ? false : operation.canTemplate,
			transferId: operation.transferId,
			operationType: operation.type
		};
		return "next";
	}
	return {
		get state() {
			return state;
		},
		get limits() {
			return limits;
		},
		get merchant() {
			return merchant;
		},
		get totalAmount() {
			return totalAmount;
		},
		setSelectedTable,
		setupCardTransfer,
		calculateCommission,
		clearTransferData,
		fineTransfer,
		changeTicketSystemByBin,
		handleChallengeSuccess,
		handleChallengeFail,
		clearTicketSystemData,
		ticketTemplateStore,
		updateAmount
	};
}
export const { getStore, initStore } = contextifyStore("TransferByCardNumberStore", makeStore);
