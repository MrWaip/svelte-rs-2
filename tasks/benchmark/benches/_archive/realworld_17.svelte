<script lang="ts">
	import '@papper-maps/map-sdk/dist/styles.css';

	import {
		ClusterGroup,
		ControlArea,
		type Coords,
		type MapSdk,
		type MapSdkEvents,
		Marker,
	} from '@papper-maps/map-sdk';
	import { useActions } from '@papper-ppr/mordor/actions/useActions';
	import { omit } from 'es-toolkit';
	import { type ComponentType, onDestroy, onMount, type SvelteComponent } from 'svelte';
	import { createEventDispatcher, getAllContexts } from 'svelte';

	import { disableBodyScroll } from '$actions/disableBodyScroll';
	import { browser } from '$app/environment';
	import { noop } from '$helpers/noop';
	import type { OptionalTrackOptions } from '$helpers/tracker';
	import { getTrackClickActionItem } from '$helpers/tracker';
	import { ctx } from '$lib/context';
	import Throbber from '$lib/Throbber/Throbber.svelte';
	import { notificationEventBus } from '$stores/notificationEventBus';
	import { overscrollEventBus } from '$stores/overscrollEventBus';
	import { getCurrentTheme } from '$stores/theme';
	import { EventAType } from '$types/enums/EventAType';
	import { Theme } from '$types/enums/Theme';
	import type { Optional } from '$types/generics/Optional';
	import type {
		ClusteredMapPoint,
		MakeMarkerFunc,
		MakeMarkerParams,
		MapItem,
		ResetMarkerFunc,
		SetActiveMarkerFunc,
	} from '$types/map';

	import { EMPTY_ID } from './constants';
	import { getStartPosition } from './helpers/helpers';
	import { makeGeoControl } from './helpers/makeGeoControl';
	import { makeMap } from './helpers/map';
	import {
		calculateClusterZoom,
		getClusterKey,
		makeDefaultMarker,
		resetActiveMarker,
		setActiveMarker,
	} from './helpers/points';

	type T = $$Generic<MapItem>;
	type MapLayersGroup = Parameters<MapSdk['removeGroup']>[0];

	export let activeItem: T | undefined = undefined;
	export let scheduledItemId: MapItem['id'] | undefined = undefined;
	export let list: T[] = [];
	export let clusteredMapPoints: ClusteredMapPoint[] = [];
	export let showRating = false;

	export let map: MapSdk | undefined = undefined;
	export let markerById: Record<string, Marker>;
	export let mobileNoticeComponent: ComponentType<SvelteComponent> | undefined = undefined;

	export let withAtmFaqControl: boolean | undefined = undefined;
	export let onMarkerClickTracker: OptionalTrackOptions = undefined;
	export let appointmentType: EventAType | undefined = EventAType.unknown;
	export let isPromo: boolean = false;
	export let getStartCords: (() => Promise<Coords>) | undefined = undefined;
	export let initialZoom: number | undefined = undefined;
	export let disableMarkerClick: boolean | undefined = undefined;
	export let useGeoControl: boolean | undefined = undefined;
	export let make: MakeMarkerFunc = makeDefaultMarker;
	export let reset: ResetMarkerFunc = resetActiveMarker;
	export let setActive: SetActiveMarkerFunc = setActiveMarker;

	const currentTheme = getCurrentTheme();
	const dispatch = createEventDispatcher<{
		rendered: null;
		onMarkerClicked: string;
		getList: null;
	}>();
	const containerId = 'papper-map-sdk-container';
	const markerTestId = 'id-pvz-marker';

	const contexts = getAllContexts();
	const isMobileWeb = ctx.deviceInfo.isMobile && !ctx.deviceInfo.isWebview;

	let clusterOfList: Optional<ClusterGroup>;
	let viewportClustersId: Record<string, string> = {};

	let showThrobber = true;

	$: if (browser && String(clusteredMapPoints.length)) {
		renderClustoredMapPoints();
		dispatch('rendered');
	}

	$: if (browser && list) {
		renderMapPoints();
		dispatch('rendered');
	}

	const onMarkerClick = (item: T) => async (eventData: MapSdkEvents.Marker.Click) => {
		if (disableMarkerClick) return;
		onMarkerClickTracker && getTrackClickActionItem(onMarkerClickTracker);
		eventData.originalEvent.preventDefault();

		if (mobileNoticeComponent) {
			notificationEventBus.openNoticeModal({
				component: mobileNoticeComponent,
				props: {
					item,
					onClose: onNoticeClose,
					onClick: noop,
					appointmentType,
					isPromo,
				},
				rounded: true,
			});
		} else {
			onNoticeClose();
		}
		activeItem = item;

		setActive(item.marker);
		dispatch('onMarkerClicked', item.id);
	};

	function renderClustoredMapPoints() {
		if (!map) {
			return;
		}

		const newClusteredMapPoints: ClusteredMapPoint[] = [];
		const pointsKeyInViewport = {};

		clusteredMapPoints.forEach((item) => {
			const key = getClusterKey(item);
			!viewportClustersId[key] && newClusteredMapPoints.push(item);
			pointsKeyInViewport[key] = key;
		});

		const removeGroups = getRemoveGroups(pointsKeyInViewport);
		removeGroups.length && map.removeGroup(removeGroups);

		const clusters: ClusterGroup[] = newClusteredMapPoints.map((item) => {
			const isMarker = item.count === 1 && item.ids.length === 1 && item.ids[0];

			if (isMarker) {
				const marker = makeMarker(
					{
						id: item.ids[0],
						coords: item.coords,
						rating: showRating ? item.rating : undefined,
					} as T,
					{
						isActive: activeItem?.id === item.ids[0],
						isScheduled: scheduledItemId === item.ids[0],
					}
				);

				return makeCluster([marker], item);
			}

			const markers = Array.from({ length: item.count }).map(() => {
				const marker = new Marker({
					testId: {
						container: markerTestId,
					},
				});
				make(marker, { ...item, id: EMPTY_ID });
				return marker;
			});
			const cluster = makeCluster(markers, item);
			cluster.subscribe('click', async (eventData) => {
				eventData.originalEvent.preventDefault();

				map?.easeTo({
					center: eventData.coords,
					zoom: calculateClusterZoom(map.zoom),
				});
			});

			return cluster;
		});
		clusters.length && map.addGroup(clusters);
	}

	function getRemoveGroups(pointsKeyInViewport: Record<string, string>) {
		const removeGroups: MapLayersGroup = [];

		Object.keys(viewportClustersId).forEach((key) => {
			if (pointsKeyInViewport[key]) {
				return;
			}

			const id = viewportClustersId[key];

			if (!id) return;

			const group = map?.getGroupById(id);

			if (group) {
				viewportClustersId = omit(viewportClustersId, [key]);
				markerById = omit(markerById, [id]);
				removeGroups.push(group);
			}
		});

		return removeGroups;
	}

	function makeCluster(markers: Marker[], item: ClusteredMapPoint) {
		const cluster = new ClusterGroup({
			markers,
			zoomOnClick: false,
		});
		viewportClustersId[getClusterKey(item)] = cluster.id;
		return cluster;
	}

	function onNoticeClose() {
		reset(activeItem?.marker, scheduledItemId === activeItem?.id);
		activeItem = undefined;
	}

	function makeMarker(
		item: T,
		{ isActive, isScheduled }: Pick<MakeMarkerParams, 'isActive' | 'isScheduled'>
	) {
		const marker = new Marker({
			testId: {
				container: markerTestId,
			},
		});
		make(marker, { ...item, isActive, isScheduled });
		marker.subscribe('click', onMarkerClick(item));

		if (activeItem && isActive) {
			activeItem.marker = marker;
		}

		markerById[item.id] = marker;
		item.marker = marker;

		return marker;
	}

	function renderMapPoints() {
		if (!map || !clusterOfList) {
			return;
		}

		const markers = list.map((item) =>
			makeMarker(item, {
				isActive: activeItem?.id === item.id,
				isScheduled: scheduledItemId === item.id,
			})
		);

		clusterOfList.addMarker(markers);
		map.addGroup(clusterOfList);
	}

	if (browser) {
		clusterOfList = new ClusterGroup({});
	}

	onMount(async () => {
		const startCoords = getStartCords ? await getStartCords() : await getStartPosition();

		map = makeMap({
			theme: $currentTheme === Theme.DARK ? 'dark' : 'light',
			startCoords,
			containerId,
			withAtmFaqControl,
			contexts,
			zoom: initialZoom,
		});
		map.subscribe('ready', async () => {
			if (map && useGeoControl) {
				makeGeoControl(map, { area: ControlArea.RC });
			}

			dispatch('getList');

			map?.subscribe('bounds-change-end', async () => {
				dispatch('getList');
			});

			showThrobber = false;
		});
	});

	overscrollEventBus.pauseOverscrollForComponent();

	onDestroy(() => {
		Object.values(viewportClustersId).forEach((id: string) => {
			const group = map?.getGroupById(id);
			group && map?.removeGroup(group);
		});
		clusterOfList && map?.getGroupById(clusterOfList.id) && map?.removeGroup(clusterOfList);

		map?.destroy();
	});
</script>

<div class="map-container" use:useActions={[disableBodyScroll]} data-testid="map-container">
	{#if showThrobber}
		<div class="loading">
			<Throbber />
		</div>
	{:else}
		<slot name="tabs" />
	{/if}
	{#if $$slots.snackbar}
		<div class="snackbar">
			<slot name="snackbar" />
		</div>
	{/if}
	<div class="map" class:mobile-web={isMobileWeb} id={containerId} />
	<div class="bottom-action">
		<slot name="bottom-action" />
	</div>
</div>

<style>
.map-container {
  position: relative;
  width: 100%;
  height: 100%;
  overflow-y: hidden;
}

.map {
  overflow: hidden;
  width: 100%;
  height: 100%;
  border-radius: 24px;
}

.map :global(.zoom-container) {
  display: grid;
  margin: 0 16px 0 0;
  background-color: transparent;
  box-shadow: unset;
  gap: 12px;
}

.map :global(.go-back-container),
.map :global(.geo-container),
.map :global(.zoom-container_plus),
.map :global(.zoom-container_minus) {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background-color: var(--layerFloor1);
  box-shadow: 0px 4px 20px 0px rgba(0, 0, 0, 0.1490196078);
  color: var(--textPrimary);
}

.map :global(.go-back-container),
.map :global(.geo-container) {
  display: flex;
  align-items: center;
  justify-content: center;
}

.map :global(.go-back-container) {
  margin: 46px 0 0 16px;
}

.map.mobile-web :global(.go-back-container) {
  margin-top: 16px;
}

.map :global(.geo-container) {
  margin: 12px 16px 0 0;
}

.map :global(.map-pin-rating) {
  display: flex;
  padding: 2px 2px 2px 8px;
  cursor: pointer;
}

.map :global(.map-pin-rating_icon) {
  display: flex;
  align-items: center;
}

.map :global(.map-pin-rating_icon > svg) {
  fill: var(--graphicRating);
}

.map :global(.map-pin-rating_value) {
  margin-left: 4px;
  color: var(--textSecondary);
}

.map :global(.marker-label_content),
.map :global(.om_-as) {
  cursor: pointer;
}

.map :global(.marker-wrapper) {
  animation: grow 0.3s 1;
}

.map :global(.marker-label_wrapper),
.map :global(.marker-label_container) {
  padding: 0 !important;
  background: none;
}

@keyframes grow {
  0% {
    transform: scale3d(0, 0, 0);
  }
  50% {
    transform: scale3d(0.5, 0.5, 0.5);
  }
  100% {
    transform: scale3d(1, 1, 1);
  }
}
.loading {
  display: flex;
  height: 100%;
  align-items: center;
  justify-content: center;
}

.snackbar {
  position: absolute;
  z-index: 1;
  top: 16px;
  right: 16px;
  width: 360px;
}

.bottom-action {
  position: absolute;
  z-index: 1;
  right: 16px;
  bottom: 16px;
  left: 16px;
}

@media (--desktop-mobile) {
  .map {
    border-radius: 0;
  }
  .snackbar {
    top: 8px;
    right: 8px;
    left: 8px;
    width: calc(100% - 16px);
  }
}
</style>
