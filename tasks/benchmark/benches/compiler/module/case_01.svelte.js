import { i18n } from "@papper-ppr-foundation/i18n";
import { failResult, okResult } from "@papper-ppr-toolbox/result";
import { head, isNil, keyBy } from "es-toolkit";
import { SvelteMap } from "svelte/reactivity";
import { apiClient } from "$gatewayService";
import { eventBus } from "$lib/eventBus";
import { field, validators } from "$lib/stores/ValidationKit";
import { asyncValidator } from "$lib/stores/ValidationKit/baseAsyncValidators";
import { ValidationKit } from "$lib/stores/ValidationKit/ValidationKit.svelte";
import { withDebounceQueue } from "$lib/synchronization/withDebounceQueue.svelte";
import { RUB_CURRENCY } from "$shared/apps/MFInternationalTransfers/constants";
import { showTransferOverLimitsCurtain } from "$shared/apps/MFInternationalTransfers/elements/curtains/TransferOverLimits";
import { CurrencyType } from "$shared/apps/MFInternationalTransfers/types";
import { formatMoneyFromCurrencyData } from "../helpers/formatMoneyFromCurrencyData";
import { ReceiverFieldsContainer } from "./ReceiverFieldsContainer.svelte";
const DEFAULT_WITHDRAW_CURRENCY = RUB_CURRENCY;
export class PayoffStore {
	constructor({ payoffId, provider, countryImageUrl, phone, clientId, availableTables, receiverFields, checkUserTransferLimits, tablesObserver }) {
		this.validation = new ValidationKit(this);
		this.updateRate = withDebounceQueue(this.updateRatesByKnownCurrencyType.bind(this));
		this.ratesAndLimitsCache = new SvelteMap();
		this.currencyMap = {};
		this.withdrawCurrency = DEFAULT_WITHDRAW_CURRENCY;
		this.transferCurrencyCode = $state("");
		this.transferAmount = $state();
		this.selectedTableNumber = $state("");
		this.lastTouchedAmountField = $state(CurrencyType.Withdraw);
		this.transferCurrency = $derived(this.currencyMap[this.transferCurrencyCode]);
		this.rate = $derived(this.ratesAndLimitsCache.get(this.transferCurrencyCode)?.value?.rate ?? this.currencyMap[this.transferCurrencyCode]?.readableRate);
		this.rateAndLimitsCacheForSelectedCurrency = $derived(this.ratesAndLimitsCache.get(this.transferCurrencyCode));
		this.withdrawAmount = field(undefined, [
			validators.number.positiveNumber(i18n("mfInternationalTransfersByPhone.formValidation.amountRequired", "lorem lorem")),
			this.validateTableBalance.bind(this),
			this.validateProviderLimits.bind(this)
		]);
		this.payoffId = payoffId;
		this.provider = provider;
		this.countryImageUrl = countryImageUrl;
		this.phone = phone;
		this.clientId = clientId;
		this.tablesObserver = tablesObserver;
		this.checkUserTransferLimits = checkUserTransferLimits;
		this.currencyMap = keyBy(provider.availableCurrencies, ({ code }) => code);
		this.transferCurrencyCode = head(provider.availableCurrencies)?.code ?? "";
		this.selectedTableNumber = head(availableTables)?.tableNumber;
		this.receiverFields = field(new ReceiverFieldsContainer(receiverFields), [asyncValidator.onSubmit(this.validateFields)]);
		this.availableTables = $derived(this.tablesObserver.tables);
		this.selectedTable = $derived(this.availableTables.find((table) => table.tableNumber === this.selectedTableNumber));
	}
	async validateFields(fields) {
		const isValid = await fields.checkIsValid();
		if (!isValid) {
			return okResult(i18n("mfInternationalTransfersByPhone.formValidation.receiverFieldsInvalidV2", "lorem lorem"));
		}
		return okResult(undefined);
	}
	validateTableBalance(value) {
		if (!value || !this.selectedTable) return;
		const cents = Math.round(value * 100);
		if (cents > this.selectedTable.balance) {
			return i18n("mfInternationalTransfersByPhone.formValidation.tableBalance", "lorem lorem lorem");
		}
	}
	validateProviderLimits(_value) {
		if (this.updateRate.isLocked) {
			return;
		}
		if (isNil(this.rateAndLimitsCacheForSelectedCurrency) || this.rateAndLimitsCacheForSelectedCurrency.isFail()) {
			return i18n("mfInternationalTransfersByPhone.formValidation.providerLimitsStaleV2", "lorem lorem");
		}
		const { min, max, withinLimits } = this.rateAndLimitsCacheForSelectedCurrency.value.limits;
		if (!withinLimits) {
			return i18n("mfInternationalTransfersByPhone.formValidation.providerLimits", "lorem {minPretty} lorem {maxPretty}", {
				minPretty: formatMoneyFromCurrencyData(min ?? 0, this.withdrawCurrency),
				maxPretty: formatMoneyFromCurrencyData(max ?? 0, this.withdrawCurrency)
			});
		}
	}
	async getRateAndLimitsData({ knownAmount, knownType }) {
		if (isNil(this.transferCurrency)) {
			return failResult(new Error("Invalid store state"));
		}
		const transferCurrencyCode = this.transferCurrency.code;
		const result = await apiClient.endpointE_updateRate.call({ input: {
			payoffID: this.payoffId,
			withdrawCurrencyCode: transferCurrencyCode,
			acceptedCurrencyCode: this.withdrawCurrency.code,
			[knownType === CurrencyType.Withdraw ? "acceptedAmount" : "withdrawAmount"]: String(knownAmount)
		} });
		this.ratesAndLimitsCache.set(transferCurrencyCode, result);
		return result;
	}
	async updateRatesByKnownCurrencyType(knownType) {
		this.lastTouchedAmountField = knownType;
		const formAmount = (knownType === CurrencyType.Withdraw ? this.withdrawAmount.value : this.transferAmount) ?? 0;
		const args = {
			knownAmount: formAmount,
			knownType
		};
		if (args.knownAmount === 0) {
			return;
		}
		const result = await this.getRateAndLimitsData(args);
		if (result.isFail()) {
			return eventBus.snackbarV2.alert(i18n("endpointE.rateCalc.failedToLoadRate", "lorem lorem lorem lorem"));
		}
		const { withdrawAmount: newWithdrawAmount, transferAmount: newTransferAmount } = result.value;
		if (knownType === CurrencyType.Withdraw) {
			this.transferAmount = newTransferAmount;
		} else {
			this.withdrawAmount.value = newWithdrawAmount;
		}
	}
	recalculateRatesUsingLastKnownInfo() {
		return this.updateRate.execute(this.lastTouchedAmountField);
	}
	resetCalculatorState() {
		this.transferAmount = undefined;
		this.withdrawAmount.value = undefined;
		this.withdrawAmount.reset();
		this.lastTouchedAmountField = CurrencyType.Withdraw;
	}
	async snapshot() {
		if (this.updateRate.isLocked) {
			return failResult("Still updating the rate");
		}
		const validationResult = await this.validation.run();
		if (validationResult.isFail() || !validationResult.value.isValid) {
			return failResult(new Error("Validation failed"));
		}
		const transferCurrency = this.transferCurrency;
		const selectedTable = this.selectedTable;
		const rate = this.rate;
		if (isNil(transferCurrency) || isNil(selectedTable) || isNil(rate)) {
			return failResult(new Error("Invalid store state"));
		}
		const { withdrawAmount, receiverFields } = validationResult.value.values;
		const { isOverLimit, type, availableLimit, transferLimits, isFullMax } = this.checkUserTransferLimits(withdrawAmount);
		if (isOverLimit) {
			showTransferOverLimitsCurtain({
				type,
				availableLimit,
				isFullMax,
				limits: transferLimits
			});
			return failResult(new Error("Transfer amount exceeds the limit"));
		}
		return okResult({
			data: {
				payoffId: this.payoffId,
				phone: this.phone,
				clientId: this.clientId,
				receiverSummaryText: receiverFields.summaryText,
				withdraw: {
					currency: this.withdrawCurrency,
					amount: withdrawAmount ?? 0
				},
				transfer: {
					currency: transferCurrency,
					amount: this.transferAmount ?? 0
				},
				table: selectedTable,
				provider: this.provider,
				countryImageUrl: this.countryImageUrl,
				rate,
				fields: receiverFields.values,
				lastTouchedAmountField: this.lastTouchedAmountField
			},
			checkUserTransferLimits: this.checkUserTransferLimits
		});
	}
}
