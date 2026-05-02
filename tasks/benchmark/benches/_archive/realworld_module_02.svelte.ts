import { i18n } from '@papper-ppr-foundation/i18n';
import { failResult, okResult, type Result } from '@papper-ppr-toolbox/result';
import { head, isNil, keyBy } from 'es-toolkit';
import { SvelteMap } from 'svelte/reactivity';

import { apiClient } from '$gatewayService';
import type { UpdateRateResult } from '$gatewayService/endpoints/internationalTransfersByPhone/updateRate';
import { eventBus } from '$lib/eventBus';
import { field, type FieldPublicApi, validators } from '$lib/stores/ValidationKit';
import { asyncValidator } from '$lib/stores/ValidationKit/baseAsyncValidators';
import { ValidationKit } from '$lib/stores/ValidationKit/ValidationKit.svelte';
import { withDebounceQueue } from '$lib/synchronization/withDebounceQueue.svelte';
import { RUB_CURRENCY } from '$shared/apps/MFInternationalTransfers/constants';
import { showTransferOverLimitsCurtain } from '$shared/apps/MFInternationalTransfers/elements/curtains/TransferOverLimits';
import type { CheckUserTransferLimitsFn } from '$shared/apps/MFInternationalTransfers/helpers/checkUserTransferLimits';
import {
	type CurrencyDescriptorExtended,
	type CurrencyShort,
	CurrencyType,
	type InternationalTransfersTable,
	type ProviderDescriptor,
	type ReadableRate,
	type ReceiverFieldSpec,
} from '$shared/apps/MFInternationalTransfers/types';

import { formatMoneyFromCurrencyData } from '../helpers/formatMoneyFromCurrencyData';
import type { TablesObserver } from './TablesObserver.svelte';
import type { FineStoreInit } from './FineStore.svelte';
import { ReceiverFieldsContainer } from './ReceiverFieldsContainer.svelte';

type InitOptions = {
	payoffId: string;
	provider: ProviderDescriptor;
	countryImageUrl: string | undefined;
	phone: string;
	clientId: string;
	availableTables: readonly InternationalTransfersTable[];
	receiverFields: readonly ReceiverFieldSpec[];
	checkUserTransferLimits: CheckUserTransferLimitsFn;
	tablesObserver: TablesObserver;
};

type RateUpdateFunctionArgs = {
	knownAmount: number;
	knownType: CurrencyType;
};

const DEFAULT_WITHDRAW_CURRENCY = RUB_CURRENCY;

export class PayoffStore {
	validation = new ValidationKit<PayoffStore>(this);

	readonly payoffId: string;
	readonly provider: ProviderDescriptor;
	readonly countryImageUrl: string | undefined;
	readonly phone: string;
	readonly clientId: string;
	readonly updateRate = withDebounceQueue(this.updateRatesByKnownCurrencyType.bind(this));
	readonly checkUserTransferLimits: CheckUserTransferLimitsFn;

	private readonly ratesAndLimitsCache = new SvelteMap<string, Result<UpdateRateResult>>();
	private readonly currencyMap: Record<string, CurrencyDescriptorExtended> = {};
	private readonly tablesObserver: TablesObserver;

	withdrawCurrency = DEFAULT_WITHDRAW_CURRENCY;

	receiverFields: FieldPublicApi<ReceiverFieldsContainer>;

	transferCurrencyCode: string = $state('');
	transferAmount: number | undefined = $state();
	selectedTableNumber: string = $state('');
	lastTouchedAmountField: CurrencyType = $state(CurrencyType.Withdraw);

	readonly availableTables: readonly InternationalTransfersTable[];
	readonly selectedTable: InternationalTransfersTable | undefined;
	readonly transferCurrency: CurrencyShort | undefined = $derived(
		this.currencyMap[this.transferCurrencyCode]
	);
	readonly rate: ReadableRate | undefined = $derived(
		this.ratesAndLimitsCache.get(this.transferCurrencyCode)?.value?.rate ??
			this.currencyMap[this.transferCurrencyCode]?.readableRate
	);
	readonly rateAndLimitsCacheForSelectedCurrency = $derived(
		this.ratesAndLimitsCache.get(this.transferCurrencyCode)
	);

	withdrawAmount = field<number | undefined>(undefined, [
		validators.number.positiveNumber(
			i18n('mfInternationalTransfersByPhone.formValidation.amountRequired', 'lorem lorem')
		),
		this.validateTableBalance.bind(this),
		this.validateProviderLimits.bind(this),
	]);

	constructor({
		payoffId,
		provider,
		countryImageUrl,
		phone,
		clientId,
		availableTables,
		receiverFields,
		checkUserTransferLimits,
		tablesObserver,
	}: InitOptions) {
		this.payoffId = payoffId;
		this.provider = provider;
		this.countryImageUrl = countryImageUrl;
		this.phone = phone;
		this.clientId = clientId;
		this.tablesObserver = tablesObserver;
		this.checkUserTransferLimits = checkUserTransferLimits;

		this.currencyMap = keyBy(provider.availableCurrencies, ({ code }) => code);

		this.transferCurrencyCode = head(provider.availableCurrencies)?.code ?? '';

		this.selectedTableNumber = head(availableTables)?.tableNumber!;

		this.receiverFields = field(new ReceiverFieldsContainer(receiverFields), [
			asyncValidator.onSubmit(this.validateFields),
		]);

		this.availableTables = $derived(this.tablesObserver.tables);
		this.selectedTable = $derived(
			this.availableTables.find((table) => table.tableNumber === this.selectedTableNumber)
		);
	}

	private async validateFields(
		fields: ReceiverFieldsContainer
	): Promise<Result<string | undefined>> {
		const isValid = await fields.checkIsValid();

		if (!isValid) {
			return okResult(
				i18n(
					'mfInternationalTransfersByPhone.formValidation.receiverFieldsInvalidV2',
					'lorem lorem'
				)
			);
		}

		return okResult(undefined);
	}

	private validateTableBalance(value: number | undefined): string | undefined {
		if (!value || !this.selectedTable) return;

		const cents = Math.round(value * 100);

		if (cents > this.selectedTable.balance) {
			return i18n(
				'mfInternationalTransfersByPhone.formValidation.tableBalance',
				'lorem lorem lorem'
			);
		}
	}

	private validateProviderLimits(_value: number | undefined): string | undefined {
		if (this.updateRate.isLocked) {
			return;
		}

		if (
			isNil(this.rateAndLimitsCacheForSelectedCurrency) ||
			this.rateAndLimitsCacheForSelectedCurrency.isFail()
		) {
			return i18n(
				'mfInternationalTransfersByPhone.formValidation.providerLimitsStaleV2',
				'lorem lorem'
			);
		}

		const { min, max, withinLimits } = this.rateAndLimitsCacheForSelectedCurrency.value.limits;

		if (!withinLimits) {
			return i18n(
				'mfInternationalTransfersByPhone.formValidation.providerLimits',
				'lorem {minPretty} lorem {maxPretty}',
				{
					minPretty: formatMoneyFromCurrencyData(min ?? 0, this.withdrawCurrency),
					maxPretty: formatMoneyFromCurrencyData(max ?? 0, this.withdrawCurrency),
				}
			);
		}
	}

	private async getRateAndLimitsData({
		knownAmount,
		knownType,
	}: RateUpdateFunctionArgs): Promise<Result<UpdateRateResult>> {
		if (isNil(this.transferCurrency)) {
			return failResult(new Error('Invalid store state'));
		}

		const transferCurrencyCode = this.transferCurrency.code;

		const result = await apiClient.endpointE_updateRate.call({
			input: {
				payoffID: this.payoffId,
				withdrawCurrencyCode: transferCurrencyCode,
				acceptedCurrencyCode: this.withdrawCurrency.code,
				[knownType === CurrencyType.Withdraw ? 'acceptedAmount' : 'withdrawAmount']:
					String(knownAmount),
			},
		});

		this.ratesAndLimitsCache.set(transferCurrencyCode, result);

		return result;
	}

	private async updateRatesByKnownCurrencyType(knownType: CurrencyType) {
		this.lastTouchedAmountField = knownType;

		const formAmount =
			(knownType === CurrencyType.Withdraw ? this.withdrawAmount.value : this.transferAmount) ?? 0;

		const args: RateUpdateFunctionArgs = {
			knownAmount: formAmount,
			knownType,
		};

		if (args.knownAmount === 0) {
			return;
		}

		const result = await this.getRateAndLimitsData(args);

		if (result.isFail()) {
			return eventBus.snackbarV2.alert(
				i18n('endpointE.rateCalc.failedToLoadRate', 'lorem lorem lorem lorem')
			);
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

	async snapshot(): Promise<
		Result<Omit<FineStoreInit, 'navigationHelpers' | 'tablesObserver'>>
	> {
		if (this.updateRate.isLocked) {
			return failResult('Still updating the rate');
		}

		const validationResult = await this.validation.run();

		if (validationResult.isFail() || !validationResult.value.isValid) {
			return failResult(new Error('Validation failed'));
		}

		const transferCurrency = this.transferCurrency;
		const selectedTable = this.selectedTable;
		const rate = this.rate;

		if (isNil(transferCurrency) || isNil(selectedTable) || isNil(rate)) {
			return failResult(new Error('Invalid store state'));
		}

		const { withdrawAmount, receiverFields } = validationResult.value.values;

		const { isOverLimit, type, availableLimit, transferLimits, isFullMax } =
			this.checkUserTransferLimits(withdrawAmount);

		if (isOverLimit) {
			showTransferOverLimitsCurtain({
				type,
				availableLimit,
				isFullMax,
				limits: transferLimits,
			});

			return failResult(new Error('Transfer amount exceeds the limit'));
		}

		return okResult({
			data: {
				payoffId: this.payoffId,
				phone: this.phone,
				clientId: this.clientId,
				receiverSummaryText: receiverFields.summaryText,
				withdraw: { currency: this.withdrawCurrency, amount: withdrawAmount ?? 0 },
				transfer: { currency: transferCurrency, amount: this.transferAmount ?? 0 },
				table: selectedTable,
				provider: this.provider,
				countryImageUrl: this.countryImageUrl,
				rate,
				fields: receiverFields.values,
				lastTouchedAmountField: this.lastTouchedAmountField,
			},
			checkUserTransferLimits: this.checkUserTransferLimits,
		} satisfies Omit<FineStoreInit, 'navigationHelpers' | 'tablesObserver'>);
	}
}
