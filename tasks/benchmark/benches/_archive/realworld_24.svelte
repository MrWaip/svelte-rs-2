<script lang="ts">
	import { IcMInfoFilled } from '@papper-ppr/mordor/icons';
	import { tweened } from '@papper-ppr-foundation/motion';
	import { debounce } from 'es-toolkit';
	import type { Writable } from 'svelte/store';
	import { writable } from 'svelte/store';

	import RadioGroup from '$components/Welcome/elements/RadioGroup.svelte';
	import {
		DEFAULT_PRICE_CENTS,
		DEFAULT_TAX_RATE,
		SIX_MONTHS_PROFITABILITY,
		SIX_MONTHS_TRADING_PERIOD_DAYS,
	} from '$constants/defaultValues';
	import { calculateOrderClient } from '$gatewayV4Service/endpoints/x/care/calcOrder';
	import { moneyFormatterFactory } from '$helpers/formatMoney';
	import { trackMount } from '$helpers/tracker';
	import { ctx } from '$lib/context';
	import { Button } from '$lib/ds/Button';
	import { emblaCarouselSvelte } from '$lib/EmblaCarousel';
	import BxMoneyInput from '$lib/MoneyInput/BxMoneyInput.svelte';
	import Tooltip from '$lib/Tooltip/Tooltip.svelte';
	import { pluralize } from '$shared/helpers/text';
	import { TAX_TOOLTIP_WIDTH } from '$stores/main/constants';
	import { getTaxTooltipText } from '$stores/main/helpers';
	import type { IssueV3 } from '$types/issuesV3';

	import { round, currStrToCents } from '../helpers';
	import { tracker } from '../tracker';

	export let issue: IssueV3 | undefined;
	export let issues: IssueV3[] | undefined;
	export let onEnterButtonClick: (value: number) => void;
	export let income: number = 0;
	export let incomeAfterTaxes: number = 0;
	export let profitability: number = 0;
	export let inputValue: Writable<number>;

	const DAYS_IN_MONTHS = 30;

	const MAX_INPUT_VALUE = 1_000_000_000;

	$: taxTooltipText = getTaxTooltipText(undefined);

	const isMobile = ctx.deviceInfo.isMobile;

	let selectedIssueId = issue?.issueId || '';

	let xyzCount: number = 100;

	const pluralizeFunction = pluralize(['%d lorem', '%d lorem', '%d lorem']);

	const getTradingPeriodMonths = () => {
		const selectedIssue = issues?.find((el) => el.issueId === selectedIssueId);

		return pluralizeFunction(
			selectedIssue
				? Math.floor(selectedIssue.tradingPeriodDays / DAYS_IN_MONTHS)
				: SIX_MONTHS_TRADING_PERIOD_DAYS / DAYS_IN_MONTHS
		);
	};

	let tradingPeriodMonths = writable(getTradingPeriodMonths());

	const animatedIncome = tweened(income);
	const animatedIncomeWithTaxes = tweened(incomeAfterTaxes);
	const animatedPercent = tweened(profitability);

	const formatMoney = moneyFormatterFactory();

	const onChangeSelected = (value: string) => {
		selectedIssueId = value;
	};

	const calculateOrder = debounce(async () => {
		if (selectedIssueId && issue && xyzCount !== 0) {
			const result = await calculateOrderClient.call({
				count: xyzCount,
				issueId: selectedIssueId,
			});

			if (!result.isOk() || !result.value) {
				return;
			}

			$inputValue = result.value.amountCents.cents;

			income = result.value.incomeCents.cents;
			incomeAfterTaxes = result.value.incomeAfterTaxesCents.cents;
			profitability = result.value.profitability / 100;
			$tradingPeriodMonths = getTradingPeriodMonths();
			taxTooltipText = getTaxTooltipText(result.value.taxPercentage);
			return;
		}

		/* lorem */
		if (!issue) {
			calculateIncome(xyzCount);
		}
	}, 500);

	$: animatedPercent.set(profitability);
	$: (xyzCount || selectedIssueId) && calculateOrder();
	$: animatedIncome.set(income);
	$: animatedIncomeWithTaxes.set(incomeAfterTaxes);

	$: badgeText = `lorem loremr ${$tradingPeriodMonths}`;

	$: forQualifiedOnly = issues?.find(
		(issue) => issue.issueId === selectedIssueId
	)?.forQualifiedOnly;

	function handleSumInput({ target, data }) {
		if (target.value === '0') {
			target.value = data;
		}

		if (target.value === '') {
			return;
		}

		const sumCents = currStrToCents(target.value);
		xyzCount = Math.floor(sumCents / DEFAULT_PRICE_CENTS);
	}

	function calculateIncome(xyzCount: number) {
		/* lorem */
		$inputValue = xyzCount * DEFAULT_PRICE_CENTS;
		/* lorem */
		const monthlyRate = SIX_MONTHS_PROFITABILITY / 12;
		/* lorem */
		income = ($inputValue * (monthlyRate / 100) * SIX_MONTHS_TRADING_PERIOD_DAYS) / 30;
		/* lorem */
		incomeAfterTaxes = income * (1 - DEFAULT_TAX_RATE / 100);
	}

	trackMount(() => tracker.view.incomeCalculatorView());
</script>

<div class="root" data-testid="income-calculation-block">
	<div class="header">lorem</div>
	{#if isMobile}
		<div class="content">
			<div class="content-section">
				<BxMoneyInput
					placeholder="loremom {formatMoney(DEFAULT_PRICE_CENTS)}"
					bind:value={$inputValue}
					dataTestid="xyz-sum-input"
					max={MAX_INPUT_VALUE}
					on:input={handleSumInput}
				/>
				<div
					class="group"
					use:emblaCarouselSvelte={{ options: { align: 'start', duration: 20 }, plugins: [] }}
				>
					<RadioGroup
						onChange={onChangeSelected}
						checkedId={selectedIssueId}
						items={issues || []}
						showQualIcon={false}
					/>
				</div>
				<div class="calculation-wrapper">
					<div class="badges">
						<div class="badge-wrapper badge-calculator" data-testid="holding-period-badge">
							{badgeText}
						</div>
						{#if forQualifiedOnly}
							<div class="badge-wrapper badge-calculator" data-testid="qualified-badge">
								loremeloremsulorem
							</div>
						{/if}
					</div>
					<div class="calculation-results">
						<div>
							<div ppr-typo="tsHeadline700XLarge" data-testid="interest-rate">
								{round($animatedPercent)}%
							</div>
							<div class="text-body" ppr-typo="tsBody400Small">loremum</div>
						</div>

						<div class="income-results">
							<div class="income-block" data-testid="income-calculator-results">
								<div ppr-typo="tsHeadline700XLarge">
									+ {formatMoney($animatedIncome)}
								</div>

								<div class="icon-info" data-testid="icon-info">
									<Tooltip contentWidth={TAX_TOOLTIP_WIDTH} placement="top">
										<IcMInfoFilled slot="activator" />
										<p>{taxTooltipText}</p>
									</Tooltip>
								</div>
							</div>

							<div
								class="text-body"
								ppr-typo="tsBody400Small"
								data-testid="income-calculator-after-tax"
							>
								{formatMoney($animatedIncomeWithTaxes)} lorem lorem
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="content">
			<div class="content-section calculator-section">
				<div class="money-input-header" ppr-typo="tsBodyControl500Medium">
					loremploremoloremtext
				</div>
				<BxMoneyInput
					placeholder="loremom {formatMoney(DEFAULT_PRICE_CENTS)}"
					bind:value={$inputValue}
					dataTestid="xyz-sum-input"
					max={MAX_INPUT_VALUE}
					on:input={handleSumInput}
				/>
				<div class="money-input-header header-margin-top" ppr-typo="tsBodyControl500Medium">
					loremsloremolor
				</div>
				<div
					class="group"
					use:emblaCarouselSvelte={{ options: { align: 'start', duration: 20 }, plugins: [] }}
				>
					<RadioGroup
						onChange={onChangeSelected}
						checkedId={selectedIssueId}
						items={issues || []}
						showQualIcon={false}
					/>
				</div>
			</div>

			<div class="content-section calculator-section">
				<div class="badges">
					<div class="badge-wrapper badge-calculator" data-testid="holding-period-badge">
						{badgeText}
					</div>
					{#if forQualifiedOnly}
						<div class="badge-wrapper badge-calculator" data-testid="qualified-badge">
							loremeloremlorem
						</div>
					{/if}
				</div>
				<div class="calculation-results">
					<div>
						<div ppr-typo="tsHeadline700XLarge" data-testid="interest-rate">
							{round($animatedPercent)}%
						</div>
						<div class="text-body" ppr-typo="tsBody400Small">loremum</div>
					</div>

					<div class="income-results">
						<div class="income-block" data-testid="income-calculator-results">
							<div ppr-typo="tsHeadline700XLarge">
								+ {formatMoney($animatedIncome)}
							</div>

							<div class="icon-info" data-testid="icon-info">
								<Tooltip contentWidth={TAX_TOOLTIP_WIDTH} placement="top">
									<IcMInfoFilled slot="activator" />
									<p>{taxTooltipText}</p>
								</Tooltip>
							</div>
						</div>

						<div
							class="text-body"
							ppr-typo="tsBody400Small"
							data-testid="income-calculator-after-tax"
						>
							{formatMoney($animatedIncomeWithTaxes)} lorem lorem
						</div>
					</div>
				</div>
				<div class="button">
					<Button
						track={tracker.click.calculateIncomeCareRequest()}
						color="actionPrimary"
						size="600"
						fullWidth
						onclick={() => onEnterButtonClick($inputValue)}
					>
						loremDlorem
					</Button>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
.root {
  display: flex;
  width: 100%;
  box-sizing: border-box;
  flex-direction: column;
  padding: 2rem;
  border-radius: 32px;
  margin-top: 2rem;
  background-color: var(--layerFloor1);
  gap: 2rem;
}

.money-input-header {
  margin-bottom: 1rem;
}

.header-margin-top {
  margin-top: 1rem;
}

.badge-wrapper {
  width: fit-content;
  padding: 3px 8px;
  border-radius: 0.5rem;
  margin-top: 1.5rem;
  background-color: rgba(204, 214, 228, 0.32);
}

.badge-calculator {
  margin-top: 0;
  margin-bottom: 1.625rem;
}

.calculation-wrapper {
  padding: 1rem;
  border: 1px solid var(--graphicNeutral);
  border-radius: 0.75rem;
}

.content {
  display: flex;
  gap: 1.5rem;
}

.content-section {
  display: flex;
  width: 100%;
  flex-direction: column;
  justify-content: space-between;
  padding: 2rem;
  border: 1px solid var(--graphicQuaternary);
  border-radius: 20px;
  background: transparent;
}

.calculation-results {
  display: flex;
  gap: 2rem;
}

.income-results {
  display: flex;
  flex-direction: column;
}

.income-block {
  display: flex;
  align-items: center;
  text-wrap: nowrap;
}

.icon-info {
  margin-left: 0.25rem;
  cursor: pointer;
  --pprIconColor: var(--graphicTertiary);
}

.button {
  margin-top: 1.5rem;
}

.text-body {
  color: var(--textSecondary);
}

.calculator-section {
  overflow: hidden;
  flex: 0 1 auto;
}

.group {
  overflow: hidden;
  padding: 0 2rem;
  margin: 0 -2rem;
}

.badges {
  display: flex;
  column-gap: 0.5rem;
}

@media (--mobile), (--desktop-mobile), (max-width: 375px) {
  .root {
    display: flex;
    width: 100%;
    padding: 0;
    margin-top: 2rem;
    background: none;
    gap: 1rem;
  }
  .content {
    flex-direction: column;
    gap: 1.5rem;
  }
  .content-section {
    display: flex;
    width: 100%;
    flex-direction: column;
    padding: 1.25rem 1rem;
    border: none;
    background-color: var(--layerFloor1);
    gap: 1rem;
  }
  .badge-wrapper {
    margin-top: 0;
  }
  .badge-calculator {
    margin-bottom: 1rem;
  }
  .calculation-results {
    gap: 2rem;
  }
  .group {
    padding: 0 1rem;
    margin: 0 -1rem;
  }
}
</style>
