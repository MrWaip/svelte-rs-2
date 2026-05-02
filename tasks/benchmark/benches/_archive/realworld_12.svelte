<script lang="ts">
	import { PPR_NAME } from '@papper-ppr/mordor/constants/brandName';
	import { IcMCross, IcMHamburger, IcSChevronRight, IcSSearchFilled } from '@papper-ppr/mordor/icons';
	import { onDestroy, onMount } from 'svelte';
	import { cubicInOut, cubicOut } from 'svelte/easing';
	import { readable } from 'svelte/store';
	import { fade, fly } from 'svelte/transition';

	import { hideEmpty } from '$actions/hideEmpty';
	import { browser } from '$app/environment';
	import { navigating, page } from '$app/stores';
	import HeaderUserMenu from '$components/HeaderUserMenu/HeaderUserMenu.svelte';
	import NavbarBadgeContainer from '$components/NavbarBadge/NavbarBadgeContainer.svelte';
	import { URL_PROFILE } from '$constants/routes';
	import { treeClientV4Client } from '$gatewayV4Service/endpoints/treeClient';
	import { companiesClient } from '$gatewayV4Service/endpoints/companies';
	import { hasSMTClient } from '$gatewayV4Service/endpoints/hasSMT';
	import { dereferenceAvatar } from '$helpers/avatar/dereferenceAvatar';
	import { isNiceTable } from '$helpers/client';
	import { formatPhone } from '$helpers/formatPhone/formatPhone';
	import { isActiveSmtLinks } from '$helpers/isActiveSmtLinks';
	import { logout, logoutFull } from '$helpers/logout/logoutHelper';
	import { goto } from '$helpers/navigation';
	import { getPapperFeatureIPageLink } from '$helpers/papperFeatureIPage';
	import { trackClickAction } from '$helpers/tracker';
	import { sendTrackingEvent } from '$helpers/tracker';
	import { pinkCompanyAvatar } from '$images';
	import ButtonBorderless from '$lib/ButtonBorderless/ButtonBorderless.svelte';
	import { ctx } from '$lib/context';
	import Counter from '$lib/Counter/Counter.svelte';
	import { getBalancesAndStatusesStore } from '$stores/balancesAndStatuses';
	import { getGoogleProductsStore } from '$stores/googleProducts';
	import { getFeaturesStore } from '$stores/features';
	import { getSessionStore } from '$stores/session';
	import { getCurrentTheme } from '$stores/theme';
	import { getUnreadCounterStore } from '$stores/unreadCounter';
	import { getUserStateStore } from '$stores/userState';
	import { ClientStatus } from '$types/enums/generated/ClientStatus';
	import { FeatureIPageLinkRefferer } from '$types/papperFeatureIPageLinkReferrer';

	import { getDefaultRouteList, smtRoutes } from '../constants';
	import { trackLinksViewOpts, trackToPapperLinkClickOpts } from '../headerTracker';
	import { tracker } from '../tracker';
	import { Routes } from '../types';
	import NavRoute from './NavRoute.svelte';

	const client = treeClientV4Client.store;
	const companies = companiesClient.store;
	const hasSMT = hasSMTClient.store;
	const session = getSessionStore();
	const features = getFeaturesStore();
	const userState = getUserStateStore();
	const theme = getCurrentTheme();
	const { fancyUser } = getBalancesAndStatusesStore();
	const { canViewAplInHeaderTab, canViewGoogleCardInHeaderTab } = getGoogleProductsStore();
	const unreadCount = $userState?.isAuthorizedInTree
		? getUnreadCounterStore().unreadCount
		: readable(0);
	const closingTableAllowedRoutesKeys = [Routes.MAIN, Routes.CHAT];
	const isDesktopMenu = ctx.deviceInfo.isDesktop || ctx.deviceInfo.isTablet;

	let activeBorderLineRef: HTMLElement | undefined;
	let routesContainerRef: HTMLElement | undefined;
	let linkRef: HTMLElement | undefined;
	let startBodyOverscrollYValue: string | undefined;
	let startBodyTouchActionValue: string | undefined;
	let menuOpened = false;

	$: if ($navigating) {
		menuOpened = false;
		resetOverflow();
	}

	$: isTreeTableClosing = $client?.status === ClientStatus.CLOSING;
	$: activeSmtLinks = isActiveSmtLinks($userState?.isClientOfTree, $hasSMT, $page.url);

	$: physName = $client?.name || formatPhone($userState?.phone);

	$: isNiceUser = isNiceTable($userState);
	$: isFancyUser = $fancyUser?.isFancy;

	$: activeSmtCompany = $session.smtCompanyId
		? $companies?.find(({ id }) => id === $session.smtCompanyId)
		: $companies?.[0];
	$: name = activeSmtLinks ? activeSmtCompany?.shortName || '' : physName;

	$: avatarUrl = activeSmtLinks
		? pinkCompanyAvatar
		: dereferenceAvatar($userState?.avatarUrl, $theme);
	$: isStrippedApp = $features.flags.strippedApp;
	$: if (menuOpened) {
		sendTrackingEvent(trackLinksViewOpts);
	}

	$: routes = getRoutes();
	$: tooManyRoutes = routes.length >= 6;
	$: burgerBtnTrack = tracker.click.burgerMenuBtn(menuOpened ? 'close' : 'open');
	$: getRoutes = () => {
		if (activeSmtLinks) {
			return smtRoutes();
		}

		const defaultRoutes = getDefaultRouteList(isDesktopMenu, isStrippedApp);

		return defaultRoutes.filter((item) => {
			if (item.key === Routes.APPLE) {
				return $canViewAplInHeaderTab;
			}

			if (item.key === Routes.GOOGLE_CARD) {
				return $canViewGoogleCardInHeaderTab;
			}

			if (isTreeTableClosing && !closingTableAllowedRoutesKeys.includes(item.key)) {
				return false;
			}

			if (item.key === Routes.SUPER) {
				return isNiceUser;
			}

			return isDesktopMenu
				? [
						Routes.MAIN,
						Routes.APPLE,
						Routes.GOOGLE_CARD,
						Routes.FANCY,
						Routes.CHAT,
						Routes.SETTINGS,
						Routes.SEARCH,
					].includes(item.key)
				: true;
		});
	};

	async function onLogoutClick() {
		await logout(fetch);
	}

	async function onFullLogoutClick() {
		await logoutFull(fetch);
	}

	function goToProfile() {
		goto(URL_PROFILE);
	}

	function resetOverflow() {
		if (!browser) {
			return;
		}

		document.body.style.overflowY = startBodyOverscrollYValue ?? '';
		document.body.style.touchAction = startBodyTouchActionValue ?? '';
	}

	function toggleMenu() {
		menuOpened = !menuOpened;

		if (menuOpened) {
			document.body.style.overflowY = 'hidden';
			document.body.style.touchAction = 'none';
		} else {
			resetOverflow();
		}
	}

	function watchActiveLinkChange() {
		if (!routesContainerRef) {
			return;
		}

		function setPosition() {
			if (!activeBorderLineRef || !linkRef) {
				return;
			}

			activeBorderLineRef.style.width = `${linkRef.clientWidth}px`;
			activeBorderLineRef.style.left = `${linkRef.offsetLeft}px`;

			mutationObserver.disconnect();
		}

		const mutationObserver = new MutationObserver(setPosition);

		setPosition();

		mutationObserver.observe(routesContainerRef, {
			childList: true,
			subtree: true,
			attributes: true,
		});

		onDestroy(() => {
			mutationObserver.disconnect();
		});
	}

	async function setBorderPosition(ref: HTMLElement) {
		linkRef = ref;
		watchActiveLinkChange();
	}

	onMount(() => {
		startBodyOverscrollYValue = document.body.style.overflowY;
		startBodyTouchActionValue = document.body.style.touchAction;
	});

	onDestroy(resetOverflow);
</script>

<div class="nav" data-testid="base-navbar" class:mobile={!isDesktopMenu}>
	<div class="mobile-menu-btn" data-testid="base-mobile-menu-btn">
		<ButtonBorderless track={burgerBtnTrack} theme="regular" on:click={toggleMenu}>
			<div class="burger-button">
				{#if !menuOpened}
					<div
						class="burger-icon"
						transition:fade={{ easing: cubicOut, duration: isDesktopMenu ? 0 : 300 }}
					>
						<IcMHamburger />
					</div>
				{:else}
					<div
						class="burger-icon close-icon"
						transition:fade={{ easing: cubicOut, duration: isDesktopMenu ? 0 : 300 }}
					>
						<IcMCross />
					</div>
				{/if}
			</div>
		</ButtonBorderless>
	</div>

	{#if isDesktopMenu || menuOpened}
		<div
			class="nav-items"
			transition:fade={{ easing: cubicInOut, duration: isDesktopMenu ? 0 : 400 }}
		>
			<div
				class="routes"
				class:too-many-routes={tooManyRoutes}
				bind:this={routesContainerRef}
				class:centered={isTreeTableClosing}
				transition:fly={{ easing: cubicInOut, y: -16, duration: isDesktopMenu ? 0 : 300 }}
			>
				{#each routes as route (route.key)}
					<div class="route">
						{#if route.key === Routes.SEARCH}
							<NavRoute
								href={route.href}
								exact={route.exact}
								clickOpts={route.clickOpts}
								onActive={setBorderPosition}
							>
								<div style:display="flex">
									<div class="search-icon">
										<IcSSearchFilled />
									</div>

									{route.text}
								</div>
							</NavRoute>
						{:else}
							<NavRoute
								href={route.href}
								exact={route.exact}
								clickOpts={route.clickOpts}
								onActive={setBorderPosition}
							>
								{route.text}

								{#if $unreadCount && route.key === Routes.CHAT}
									<Counter count={$unreadCount} />
								{/if}
							</NavRoute>
						{/if}
					</div>
				{/each}
				<div class="active-item-line" bind:this={activeBorderLineRef} />
			</div>
		</div>
	{/if}

	<div class="badge" use:hideEmpty>
		<NavbarBadgeContainer />
	</div>

	<a
		class="papper"
		class:too-many-routes={tooManyRoutes}
		use:trackClickAction={trackToPapperLinkClickOpts}
		href={getPapperFeatureIPageLink(FeatureIPageLinkRefferer.header)}
		target="_blank"
	>
		Lorem {PPR_NAME}
		<span class="icon">
			<IcSChevronRight />
		</span>
	</a>

	<div class="user-label-desktop">
		<HeaderUserMenu
			{name}
			{avatarUrl}
			isFancyUser={isFancyUser && !isNiceUser}
			onExitClick={onLogoutClick}
			onSettingsClick={goToProfile}
			onFullExitClick={onFullLogoutClick}
			inNewView
		/>
	</div>
</div>

<style>
	.nav {
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		padding-top: 0.1875rem;
		background: var(--headerBackground, var(--layerFloor1));
		font-size: var(--bodyLFont);
	}

	.nav-items {
		height: 100%;
		padding: 0 0.125rem;
		margin-right: auto;
	}

	.routes {
		position: relative;
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: flex-start;
		padding: 0 0.125rem;
		margin-left: 2.125rem;
		gap: 2rem;
	}

	.routes.centered {
		justify-content: center;
		gap: 1.5rem;
	}

	.search-icon {
		display: flex;
		align-items: center;
		margin-right: 0.5rem;
	}

	.route {
		height: 100%;
	}

	.badge {
		margin: 0 0.5rem 0 1.5rem;
	}

	.active-item-line {
		position: absolute;
		bottom: 0;
		left: 0;
		width: 0px;
		height: 2px;
		background-color: var(--textPrimary);
		transition:
			left 0.3s ease-out,
			width 0.3s ease-out;
	}

	.papper {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 1rem;
		margin-right: 1.25rem;
		white-space: nowrap;
	}

	.papper .icon {
		margin-top: 0.3125rem;
		margin-left: 0.375rem;
	}

	.mobile-menu-btn {
		display: none;
	}

	.burger-button {
		position: relative;
		display: flex;
		width: 1.5rem;
		height: 1.5rem;
	}

	.burger-icon {
		position: absolute;
		top: 0;
		right: 0;
		color: var(--textPrimary);
	}

	.mobile.nav {
		overflow: hidden;
		height: 100%;
		flex-direction: column;
	}

	.mobile .mobile-menu-btn {
		position: relative;
		display: flex;
		min-height: 100%;
		margin-left: auto;
	}

	.mobile .user-label-desktop {
		display: none;
	}

	.mobile .nav-items {
		position: fixed;
		z-index: 100;
		top: var(--appHeaderHeight);
		right: 0;
		bottom: 0;
		left: 0;
		height: unset;
		align-items: flex-start;
		padding: 0 1rem;
		margin: unset;
		background: var(--layerFloor1);
	}

	.mobile .routes {
		display: flex;
		width: 100%;
		height: auto;
		flex-direction: column;
		align-items: flex-start;
		justify-content: space-between;
		padding: 1.5625rem 0.125rem 0;
		margin: 0 0 1.5rem;
		font-size: var(--bodyXLFont);
		font-weight: var(--fontNormal);
		gap: 1.375rem;
	}

	.mobile .active-item-line {
		display: none;
	}

	.mobile .papper {
		display: none;
	}

	@media (--desktop-mobile-nav) {
		.routes {
			margin-left: 0;
			gap: 0.5rem;
		}

		.papper {
			margin-right: 0;
		}
	}

	@media (--desktop-mobile-nav-wide) {
		.routes.too-many-routes {
			margin-left: 0;
			gap: 0.5rem;
		}

		.papper.too-many-routes {
			margin-right: 0;
		}
	}
</style>
