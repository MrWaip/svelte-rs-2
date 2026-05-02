<script lang="ts">
	import { i18n } from '@papper-ppr-foundation/i18n';
	import { bridgeApi } from '@papper-ppr-mobile-integrations/native-bridge';
	import {
		IcMChevronRightFilled,
		IcMCircleQuestionFilled,
		IcMShareAltFilled,
	} from '@papper-ppr-ds/icons';

	import Button from '$components/Button/Button.svelte';
	import Cell from '$components/CellV2/Cell.svelte';
	import CellBox from '$components/CellV2/CellBox.svelte';
	import CellContainer from '$components/CellV2/CellContainer.svelte';
	import CellContent from '$components/CellV2/CellContent.svelte';
	import Header from '$components/Header/Header.svelte';
	import Icon from '$components/Icon/Icon.svelte';
	import Image from '$components/Image/Image.svelte';
	import FinalStepView from '$components/LightTables/CreateAndAddFlow/FinalStepView/FinalStepView.svelte';
	import MobileOnlyCloseButton from '$components/NavButtons/MobileOnlyCloseButton.svelte';
	import { LIGHT_TABLE_HELP_URL } from '$constants/routes';
	import {
		JointTableProductType,
		JointTableProductTypeInput,
	} from '$gatewayService/__queryTypes';
	import { clientLogger } from '$lib/clientLogger';
	import { eventBus } from '$lib/eventBus';
	import importantInformationSm from '$shared/LightTables/CreateAndAddMember/images/important_information_sm.webp';
	import { lightTablePromoUpgradeNavigation } from '$shared/navigation/lightTablePromoUpgrade';

	import { makeTracker } from './tracker';

	interface Props {
		onCloseClick: () => void;
		onTryAgain?: VoidFunction;
		onContinueWithoutGuest?: VoidFunction;
		fullname: string;
		trackName: string;
		productType: JointTableProductType | JointTableProductTypeInput;
	}

	let {
		onCloseClick,
		trackName,
		fullname,
		onContinueWithoutGuest,
		onTryAgain,
		productType,
	}: Props = $props();

	const tracker = $derived(makeTracker(trackName));

	async function handleShare() {
		const link = lightTablePromoUpgradeNavigation(productType);

		let intro: string;

		switch (productType) {
			case JointTableProductType.PRODUCT_TYPE_PPR_FAMILY:
				intro = i18n(
					'components.failNeedUpgrade.share.message.introFamily',
					'Lorem ipsum dolor sit text consectetur adipiscing elit Papper Banco. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.'
				);
				break;
			default:
				intro = i18n(
					'components.failNeedUpgrade.share.message.introDefault',
					'Lorem ipsum dolor sit text consectetur sample title in Papper Banco. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.'
				);
				break;
		}

		const message = [
			intro,
			'',
			i18n(
				'components.failNeedUpgrade.share.message.instruction',
				'Lorem ipsum dolor sit text, consectetur in sample / or default content.'
			),
			'',
			`${link}`,
		].join('\n');

		const result = await bridgeApi.navigation.share_plain_text({
			text: message,
		});

		if (result.isOk()) return;

		clientLogger.error(new Error('fail to share though native_bridge', { cause: result.error }));

		try {
			return await navigator.share({ text: message });
		} catch (cause) {
			clientLogger.error(new Error('fail to share though navigator', { cause }));
		}

		return eventBus.snackbarV2.alert(
			i18n(
				'components.failNeedUpgrade.error.failedToShareWithMemberV2',
				'Lorem ipsum dolor sit text'
			)
		);
	}
</script>

<FinalStepView variant="animated" dataTestId="error-invite-guest-container">
	{#snippet header()}
		<Header>
			{#snippet right()}
				<MobileOnlyCloseButton onclick={onCloseClick} track={tracker.click.close()} />
			{/snippet}
		</Header>
	{/snippet}

	{#snippet image()}
		<Image resizing="fixed" containerWidth={132} src={importantInformationSm} alt="Lorem ipsum" />
	{/snippet}

	{#snippet content()}
		<div ppr-typo="tsHeadline600Large">
			{i18n(
				'apps.createAndAddFlow.failResultNeedUpgrade.title',
				'{fullname} lorem ipsum{NBSP}dolor sit text',
				{ fullname: fullname }
			)}
		</div>

		<div ppr-typo="tsBody500Medium" class="text">
			{i18n(
				'apps.createAndAddFlow.failResultNeedUpgrade.descriptionV2',
				'Lorem dolor consectetur or sample title de default'
			)}
		</div>

		<div class="cells">
			<CellContainer yPadding={12} xPadding={16} yGap={8} xGap={12} separator={false}>
				<Cell track={tracker.click.noop()}>
					<CellBox>
						<Icon text="1" bgColor="layerFloor1" color="textAction" size="500" shape="squircle" />
					</CellBox>

					<CellBox>
						<CellContent
							titleNumberOfLines={2}
							title={i18n(
								'apps.createAndAddFlow.failResultNeedUpgrade.suggestion.titleV2',
								'Lorem default cum content'
							)}
						/>
					</CellBox>
				</Cell>

				<Cell track={tracker.click.noop()}>
					<CellBox>
						<Icon text="2" bgColor="layerFloor1" color="textAction" size="500" shape="squircle" />
					</CellBox>

					<CellBox>
						<CellContent
							titleNumberOfLines={2}
							title={i18n(
								'apps.createAndAddFlow.failResultNeedUpgrade.retryInstructionV2',
								'Lorem item value, name id sample title'
							)}
						/>
					</CellBox>
				</Cell>
			</CellContainer>

			<div class="info-cell">
				<CellContainer yPadding={12} xPadding={16} yGap={8} xGap={12} separator={false}>
					<Cell
						attrs={{ target: '_blank' }}
						href={LIGHT_TABLE_HELP_URL.toString()}
						track={tracker.click.help()}
					>
						<CellBox>
							<Icon
								icon={IcMCircleQuestionFilled}
								bgColor="layerFloor1"
								color="textAction"
								size="500"
								shape="squircle"
							/>
						</CellBox>

						<CellBox>
							<CellContent
								titleNumberOfLines={2}
								dataTestid="more-about-cool-table-button"
								title={i18n(
									'apps.createAndAddFlow.failResultNeedUpgrade.link.title',
									'Lorem ipsum dolor sit text'
								)}
							/>
						</CellBox>

						<CellBox>
							<Icon icon={IcMChevronRightFilled} color="graphicTertiary" size="500" shape="none" />
						</CellBox>
					</Cell>
				</CellContainer>
			</div>
		</div>

		<div class="actions">
			<Button
				size="600"
				fullWidth
				icon={IcMShareAltFilled}
				color="actionPrimary"
				track={tracker.click.share()}
				onclick={handleShare}
				dataTestid="share-offer-button"
				>{i18n(
					'apps.createAndAddFlow.failResultNeedUpgrade.shareOffer.labelV2',
					'Lorem ipsum'
				)}</Button
			>

			{#if onTryAgain}
				<Button
					size="600"
					fullWidth
					color="actionSecondary"
					track={tracker.click.tryAgain()}
					onclick={onTryAgain}
					dataTestid="accept-button"
				>
					{i18n('apps.createAndAddFlow.failResultNeedUpgrade.understoodButton.label', 'Lorem')}
				</Button>
			{/if}

			{#if onContinueWithoutGuest}
				<Button
					size="600"
					fullWidth
					color="actionSecondary"
					track={tracker.click.continueWithoutGuest()}
					onclick={onContinueWithoutGuest}
					dataTestid="continue-button"
				>
					{i18n(
						'apps.createAndAddFlow.failResultNeedUpgrade.createWithoutMember.label',
						'Lorem ipsum dolor sit text'
					)}
				</Button>
			{/if}
		</div>
	{/snippet}
</FinalStepView>

<style>
.text {
  margin-top: 0.5rem;
  margin-bottom: 2rem;
  color: var(--textSecondary);
}

.actions {
  display: flex;
  flex-direction: column;
  margin-top: 2rem;
  gap: 0.75rem;
}

.cells {
  text-align: left;
}

.info-cell {
  margin-top: 0.5rem;
}
</style>
