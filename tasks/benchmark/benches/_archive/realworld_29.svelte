<script lang="ts">
	import { FANCY_NAME } from '@papper-ppr/mordor/constants/brandName';
	import { IcMStatusPointsFilled, IcSChevronRightFilled, IcSCross } from '@papper-ppr/mordor/icons';
	import { bridgeApi } from '@papper-ppr-mobile-integrations/native-bridge';
	import { Navigation } from '@papper-ppr-mobile-integrations/native-bridge/navigation';
	import { onDestroy } from 'svelte';

	import { browser } from '$app/environment';
	import NavbarBadgeContainer from '$components/NavbarBadge/NavbarBadgeContainer.svelte';
	import SearchIconButton from '$components/SearchIconButton/SearchIconButton.svelte';
	import { URL_PROFILE } from '$constants/routes';
	import { treeClientV4Client } from '$gatewayV4Service/endpoints/treeClient';
	import { dereferenceAvatar } from '$helpers/avatar/dereferenceAvatar';
	import { isNiceTable } from '$helpers/client';
	import { formatPhone } from '$helpers/formatPhone/formatPhone';
	import { ctx } from '$lib/context';
	import { Badge } from '$lib/ds/Badge';
	import { IconButton } from '$lib/ds/IconButton';
	import { getBalancesAndStatusesStore } from '$stores/balancesAndStatuses';
	import { getFeaturesStore } from '$stores/features';
	import { getCurrentTheme } from '$stores/theme';
	import { getUserStateStore } from '$stores/userState';
	import { Theme } from '$types/enums/Theme';

	import NiceBadgeDark from '../images/super-badge-dark.svg';
	import NiceBadgeDefault from '../images/super-badge-default.svg';
	import { tracker } from '../tracker';

	export let fixed: boolean;
	export let redesigned: boolean = false;

	// scroll boundary for collapse/expand header
	const scrolledDownTheshold = 100;
	const scrolledUpTheshold = 10;

	// same as margin from expanded header to cards
	const redesignedScrolledTheshold = 4;

	const userState = getUserStateStore();
	const features = getFeaturesStore();
	const { fancyUser, fancyUserLoading } = getBalancesAndStatusesStore();
	const theme = getCurrentTheme();
	const client = treeClientV4Client.store;
	const isStrippedApp = $features.flags.strippedApp;

	$: name = $client?.name || formatPhone($client?.phone);
	$: avatarUrl = dereferenceAvatar($userState?.avatarUrl, $theme);
	$: isPapperApp = ['papper', 'select', 'travel'].includes(String(ctx.appHeaders.xAppName));
	$: isFancy = $fancyUser?.isFancy;
	$: isNice = isNiceTable($userState);

	$: waitingFancyLoad = !isNice && $fancyUserLoading;
	$: isNiceOrFancy = isFancy || isNice;

	let premAnimated = false;
	$: if (waitingFancyLoad) {
		premAnimated = true;
	}

	let scrolled = false;

	function calcScrolled(scrollY: number) {
		if (redesigned) {
			if (scrollY < redesignedScrolledTheshold) {
				return false;
			}

			return true;
		}

		if (!fixed || scrollY <= scrolledUpTheshold) {
			return false;
		} else if (scrollY > scrolledDownTheshold) {
			return true;
		} else {
			return scrolled;
		}
	}

	let scrollY = 0;
	$: scrolled = calcScrolled(scrollY);

	$: dynamicStatusBar =
		redesigned &&
		ctx.deviceInfo.isAndroidApp &&
		$theme === Theme.DEFAULT &&
		bridgeApi?.appearance?.system_bars_appearance;

	$: if (dynamicStatusBar && !scrolled) {
		bridgeApi.appearance.system_bars_appearance({ lightStatus: false, lightNavigation: true });
	}
	$: if (dynamicStatusBar && scrolled) {
		bridgeApi.appearance.system_bars_appearance({ lightStatus: true, lightNavigation: true });
	}

	onDestroy(() => {
		if (!browser) return;

		// reset
		if (dynamicStatusBar) {
			bridgeApi.appearance.system_bars_appearance({ lightStatus: true, lightNavigation: true });
		}

		const head = document.querySelector('head');
		const userHeaderStyle = document.querySelector('style[data-component=userHeader]');
		const userHeaderPaddingStyle = document.querySelector(
			'style[data-component=userHeaderPadding]'
		);
		if (!head) {
			return;
		}

		if (userHeaderStyle) head.removeChild(userHeaderStyle);
		if (userHeaderPaddingStyle) head.removeChild(userHeaderPaddingStyle);
	});
</script>

<svelte:window bind:scrollY />

<div class="header" class:fixed class:redesigned class:scrolled>
	<div class="user-header" class:scrolled>
		<a class="user" href={URL_PROFILE}>
			<div class="avatar">
				<img alt="" src={avatarUrl} width="34" loading="lazy" />
			</div>

			<div class="link" class:centered={!isNiceOrFancy}>
				<span class="name" ppr-typo="tsBodyControl400Small">
					{name}
				</span>
				<div class="chevron">
					<IcSChevronRightFilled />
				</div>
			</div>
			{#if isNiceOrFancy}
				<div class="logo" class:premAnimated class:hidden={waitingFancyLoad}>
					{#if isNice}
						<img
							class="super-badge"
							class:redesigned
							class:scrolled
							src={$theme === Theme.DARK || (redesigned && !scrolled)
								? NiceBadgeDark
								: NiceBadgeDefault}
							alt=""
							width="42"
							loading="lazy"
						/>
					{:else if isFancy}
						<span class="icon-fancy" class:scrolled>
							<Badge
								size="300"
								color={scrolled ? 'customFancy' : 'actionSecondary'}
								text={FANCY_NAME}
								icon={IcMStatusPointsFilled}
							/>
						</span>
					{/if}
				</div>
			{/if}
		</a>

		<div class="buttons">
			{#if !$features.flags.strippedApp}
				<NavbarBadgeContainer {scrolled} {redesigned} />
			{/if}

			{#if !isStrippedApp}
				<div class="search-icon" data-testid="search-icon">
					<SearchIconButton />
				</div>
			{/if}

			{#if isPapperApp}
				<span class="close-icon" class:scrolled>
					<IconButton
						track={tracker.click.HeaderIconCloseMiniapp()}
						size="400"
						shape="circle"
						icon={IcSCross}
						color="customGray"
						onclick={Navigation.closeWindow}
						{...{
							'aria-label': 'close-app',
						}}
					/>
				</span>
			{/if}
		</div>
	</div>
</div>

<svelte:head>
	{#if fixed}
		<style data-component="userHeader">
			#svelte {
				padding-top: var(--appTopbarAndPaddingHeight, 0px);
				background-color: var(--layerFloor0);
			}
		</style>

		{#if redesigned}
			<style data-component="userHeaderPadding">
				:root {
					--appTopbarAndPaddingHeight: 0;
				}
			</style>
		{:else}
			<style data-component="userHeaderPadding">
				:root {
					--appTopbarAndPaddingHeight: calc(var(--appPaddingTop, 0px) + 2.75rem + 1rem);
				}
			</style>
		{/if}
	{/if}
</svelte:head>

<style>
.header {
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  align-items: center;
  justify-content: space-between;
  border-radius: 0 0 1.5rem 1.5rem;
  background-color: var(--layerFloor1);
  background-position: 50% 100%;
  background-size: cover;
  transition: background-color ease-in-out 300ms;
}
.header.fixed {
  position: fixed;
  z-index: var(--zIndexToolbar);
  top: 0;
  width: 100%;
  padding-top: var(--appPaddingTop, 0);
}
.header.fixed.redesigned {
  background-color: transparent;
  transition: background-color 150ms ease-in-out 150ms;
}
.header.fixed.redesigned.scrolled {
  background-color: var(--layerFloor1);
  transition: background-color 150ms ease-in-out;
}

.user-header {
  display: grid;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  padding: 0.25rem 1rem 0.75rem;
  gap: 0.5rem;
  grid-auto-flow: column;
}

.user {
  display: grid;
  align-content: flex-start;
  align-items: center;
  justify-content: flex-start;
  color: var(--graphicPrimary);
  gap: 0.125rem 0.5rem;
  grid-template-areas: "ava name" "ava logo";
  grid-template-columns: auto 1fr auto;
  grid-template-rows: 1fr 1fr;
  /* stylelint-disable-next-line plugin/no-unsupported-browser-features */
  transition: grid-template-rows 300ms ease-in-out, gap 300ms ease-in-out;
}
.scrolled .user {
  gap: 0 0.5rem;
}
.redesigned .user {
  /* stylelint-disable-next-line plugin/no-unsupported-browser-features */
  transition: grid-template-rows 150ms ease-in-out, gap 150ms ease-in-out;
}
.redesigned.scrolled .user {
  /* stylelint-disable-next-line plugin/no-unsupported-browser-features */
  transition: grid-template-rows 150ms ease-in-out 150ms, gap 150ms ease-in-out 150ms;
}

.avatar {
  display: flex;
  align-self: flex-start;
  grid-area: ava;
}
.avatar > img {
  width: 2.75rem;
  height: 2.75rem;
  border-radius: 0.75rem;
  transition: height ease-in-out 300ms, width ease-in-out 300ms;
}
.scrolled .avatar > img {
  width: 2.125rem;
  height: 2.125rem;
}
.redesigned .avatar > img {
  transition: height 150ms ease-in-out, width 150ms ease-in-out;
}
.redesigned.scrolled .avatar > img {
  transition: height 150ms ease-in-out 150ms, width 150ms ease-in-out 150ms;
}

.link {
  display: grid;
  align-self: flex-end;
  color: var(--graphicPrimary);
  grid-area: name;
  grid-auto-flow: column;
  grid-template-columns: minmax(auto, min-content) auto;
  transform: translate3d(0, 0rem, 0);
  transition: transform 300ms ease-in-out;
}
.link.centered {
  transform: translate3d(0, 0.6rem, 0);
}
.redesigned .link {
  transition: transform 150ms ease-in-out;
}
.redesigned.scrolled .link {
  transition: transform 150ms ease-in-out 150ms;
}
.link .name {
  overflow: hidden;
  font-size: 0.875rem;
  /* FONT: compact-control/400-small */
  line-height: 1rem;
  text-overflow: ellipsis;
  transition: color 300ms ease-in-out;
  white-space: nowrap;
}
.redesigned .link .name {
  transition: color 150ms ease-in-out 150ms;
}
.redesigned.scrolled .link .name {
  transition: color 150ms ease-in-out;
}
.redesigned:not(.scrolled) .link .name {
  color: var(--textLightKey);
}
.link .chevron {
  display: flex;
  color: var(--graphicTertiary);
  transition: color 300ms ease-in-out;
}
.redesigned .link .chevron {
  transition: color 150ms ease-in-out 150ms;
}
.redesigned.scrolled .link .chevron {
  transition: color 150ms ease-in-out;
}
.redesigned:not(.scrolled) .link .chevron {
  color: var(--clearLightKey500);
}

.logo {
  display: grid;
  max-height: 1rem;
  align-self: flex-start;
  justify-content: flex-start;
  gap: 0.25rem;
  grid-area: logo;
  grid-auto-flow: column;
  opacity: 1;
}
.logo.premAnimated {
  animation: autoHeight 300ms linear;
}
.redesigned .logo.premAnimated {
  animation: autoHeight 150ms linear;
}
.logo.hidden {
  display: none;
}
.logo .super-badge.redesigned {
  border-radius: 5px;
  background-color: var(--bgFancyPrimary);
}
.logo .super-badge.redesigned:not(.scrolled) {
  background-color: black;
}

.buttons {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 0.5rem;
}
.buttons > :global(*) {
  flex-shrink: 0;
}
.buttons .search-icon {
  width: 2rem;
  height: 2rem;
}
.redesigned:not(.scrolled) .buttons .search-icon {
  --pprIconColor: var(--bgLightKey);
}

.redesigned .close-icon:not(.scrolled) {
  --ppriIconButtonBgColor: var(--graphicTertiaryOnDark);
  --ppriIconButtonColor: var(--graphicLightKey);
  border-radius: 50%;
}

.redesigned .icon-fancy:not(.scrolled) {
  --_customColor: var(--textActionOnLight);
  --_customBgColor: var(--bgLightKey);
}

@keyframes autoHeight {
  from {
    max-height: 0rem;
    opacity: 0;
  }
}
</style>
