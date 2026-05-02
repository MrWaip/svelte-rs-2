<script context="module" lang="ts">
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const { Story } = defineMeta({
		title: 'UI-KIT/Cell',
		// 👇 Disable auto-generated documentation for this component
		tags: ['!autodocs'],
	});
</script>

<script lang="ts">
	import {
		IcLAddWarranty,
		IcMChevronRightFilled,
		IcMChildFilled,
		IcMFlashFilled,
		IcMHeartFilled,
		IcMInfoFilled,
	} from '@papper-ppr-ds/icons';

	import Cell from '../ui/Cell.svelte';
	let radioGroup = '1';
</script>

<Story exportName="view" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="main">
			<Cell
				left={{
					iconProps: {
						icon: IcLAddWarranty,
						color: 'graphicActionPrimary',
						bgColor: 'bgSecondary',
						shape: 'squircle',
					},
					size: 'l',
				}}
				center={{ type: 'default-badge', badgeProps: { color: 'customFancy', text: 'jedi' } }}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
		</div>
	{/snippet}
</Story>
<Story exportName="center" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="grid">
			<Cell
				center={{ type: 'default', size: '400' }}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
			<Cell
				center={{ type: 'default-badge', badgeProps: { color: 'customFancy', text: 'lorem' } }}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
		</div>
	{/snippet}
</Story>
<Story exportName="left" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="grid">
			<Cell
				left={{
					iconProps: {
						text: '42',
						color: 'graphicActionPrimary',
						bgColor: 'bgSecondary',
						shape: 'circle',
					},
					size: 'l',
				}}
				center={{ type: 'default-badge', badgeProps: { color: 'customFancy', text: 'jedi' } }}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
			<Cell
				left={{
					iconProps: {
						icon: IcMHeartFilled,
						color: 'graphicActionPrimary',
						bgColor: 'layerFloor1',
						shape: 'squircle',
					},
					size: 'l',
				}}
				center={{
					type: 'default',
					size: '500',
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle"
					>loremiloremdloremrloremtloremcloremtulorem.lorem Wlorempisodloremlorem
					- Revenge of the Sith</svelte:fragment
				>
			</Cell>
			<Cell
				left={{
					iconProps: {
						icon: IcMFlashFilled,
						color: 'graphicActionPrimary',
						bgColor: 'layerFloor1',
						shape: 'squircle',
					},
					size: 'l',
				}}
				leftIsTop
				center={{
					type: 'default',
					size: '500',
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">loremmlorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
		</div>
	{/snippet}
</Story>
<Story exportName="leftRadio" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="grid">
			<div class="container list">
				<Cell
					left={{
						radioProps: {
							value: '1',
							id: 'radio-1',
						},
					}}
					center={{ type: 'default', size: '400' }}
					verticalPaddings="8px"
					rightPadding="16px"
					leftPadding="16px"
					color="neutral"
					separatorAuto
					bind:group={radioGroup}
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
				</Cell>
				<Cell
					left={{
						radioProps: {
							value: '2',
							id: 'radio-2',
						},
						iconProps: {
							icon: IcMHeartFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
							shape: 'squircle',
						},
						size: 'm',
					}}
					center={{ type: 'default', size: '400' }}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					color="neutral"
					separatorAuto
					bind:group={radioGroup}
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
				</Cell>
			</div>
		</div>
	{/snippet}
</Story>
<Story exportName="leftFlag" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="grid">
			<Cell
				left={{ checkboxProps: { checked: false, id: 'checkbox-777' } }}
				center={{
					type: 'default',
					size: '500',
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
		</div>
	{/snippet}
</Story>
<Story exportName="right" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="grid">
			<Cell
				center={{
					type: 'default',
					size: '500',
				}}
				right={{
					iconProps: {
						icon: IcMChevronRightFilled,
						color: 'graphicActionPrimary',
						bgColor: 'layerFloor1',
					},
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
				href="//www.papper.ru"
				target="_blank"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
			<Cell
				center={{
					type: 'default-icon',
					iconProps: {
						icon: IcMChildFilled,
						color: 'graphicActionPrimary',
						bgColor: 'layerFloor1',
					},
				}}
				right={{
					badgeProps: { color: 'customFancy', text: 'lorem' },
					toggleProps: { checked: false, id: 'toggle-777' },
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
			<Cell
				center={{
					type: 'default',
					size: '500',
				}}
				right={{
					buttonProps: { text: 'lorem', color: 'accentPrimary', id: 'button-777' },
					onclick: () => {
						alert('click right');
					},
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
			<Cell
				center={{
					type: 'default',
					size: '500',
				}}
				right={{
					buttonProps: {
						text: 'lorem',
						color: 'accentPrimary',
						id: 'button-888',
						disabled: true,
					},
					onclick: () => {
						alert('click right');
					},
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
			<Cell
				right={{
					checkboxProps: { checked: false, id: 'checkbox-77fd8' },
				}}
				center={{
					type: 'default',
					size: '500',
				}}
				verticalPaddings="12px"
				rightPadding="16px"
				leftPadding="16px"
				borderRadius="16px"
				color="neutral"
			>
				<svelte:fragment slot="title">lorem</svelte:fragment>
				<svelte:fragment slot="subtitle">lorem</svelte:fragment>
			</Cell>
		</div>
	{/snippet}
</Story>
<Story exportName="variants" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="usage">
			<div class="container">
				<Cell
					left={{
						iconProps: {
							icon: IcMHeartFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
							shape: 'squircle',
						},
						size: 'm',
					}}
					center={{
						type: 'default',
						size: '500',
					}}
					right={{
						iconProps: {
							icon: IcMChevronRightFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
						},
					}}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					borderRadius="16px"
					color="neutral"
					href="//www.papper.ru"
					target="_blank"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					left={{
						iconProps: {
							image: example.com,
							bgColor: 'layerFloor1',
							shape: 'squircle',
						},
						size: 'xl',
					}}
					center={{
						type: 'default',
						size: '500',
					}}
					right={{
						badgeProps: { color: 'actionPrimary', text: 'lorem' },
					}}
					onclick={() => alert('click center')}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					borderRadius="16px"
					color="neutral"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					left={{
						checkboxProps: {
							checked: false,
							id: 'checkbox-1',
						},
					}}
					center={{
						type: 'accent',
						size: '400',
					}}
					right={{
						indicatorProps: { text: 'EN', color: 'accent', size: '700' },
					}}
					onclick={() => alert('click center')}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					borderRadius="16px"
					color="neutral"
				>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					left={{
						iconProps: {
							icon: IcMChildFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
							shape: 'squircle',
						},
						checkboxProps: {
							checked: false,
							id: 'checkbox-2',
						},
						size: 'l',
					}}
					center={{
						type: 'revert',
						size: '400',
					}}
					right={{ toggleProps: { checked: false, id: 'toggle-1afsdf' } }}
					onclick={() => alert('click center')}
					verticalPaddings="12px"
					rightPadding="8px"
					leftPadding="8px"
					borderRadius="16px"
					color="neutral"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container list">
				<Cell
					left={{
						radioProps: {
							value: '1',
							id: 'radio-1',
						},
					}}
					center={{
						type: 'default-icon',
						iconProps: {
							icon: IcMChildFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
						},
					}}
					right={{
						buttonProps: { text: 'lorem', color: 'actionPrimary', id: 'button-1' },
					}}
					verticalPaddings="8px"
					rightPadding="16px"
					leftPadding="16px"
					color="neutral"
					separatorAuto
					bind:group={radioGroup}
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
				<Cell
					left={{
						radioProps: {
							value: '2',
							id: 'radio-2',
						},
						iconProps: {
							icon: IcMHeartFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
							shape: 'squircle',
						},
						size: 'm',
					}}
					center={{
						type: 'default-badge',
						badgeProps: { color: 'customFancy', text: 'lorem' },
					}}
					right={{
						badgeProps: { color: 'actionPrimary', text: 'lorem' },
						indicatorProps: { icon: IcMChildFilled, color: 'accent', size: '600' },
					}}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					color="neutral"
					separatorAuto
					bind:group={radioGroup}
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					center={{
						type: 'badge-list',
						badgeProps: [
							{ color: 'customFancy', text: 'lorem1' },
							{ color: 'negativePrimary', text: 'lorem2' },
						],
					}}
					right={{
						badgeProps: { color: 'actionPrimary', text: 'lorem' },
						indicatorProps: { icon: IcMChildFilled, color: 'accent', size: '600' },
					}}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					borderRadius="16px"
					color="neutral"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>

			<div class="container">
				<Cell
					center={{
						type: 'default',
						size: '500',
					}}
					right={{
						iconProps: {
							icon: IcMChevronRightFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
						},
						badgeProps: { color: 'negativePrimary', text: 'lorem' },
					}}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					centralPadding="10px"
					borderRadius="16px 0 16px 0"
					color="neutral"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					center={{
						type: 'badge-list',
						badgeProps: [
							{ color: 'customFancy', text: 'lorem1' },
							{ color: 'negativePrimary', text: 'lorem2' },
						],
					}}
					right={{
						iconProps: {
							icon: IcMChevronRightFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
						},
						indicatorProps: { icon: IcMChildFilled, color: 'accent' },
					}}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					borderRadius="16px"
					color="transparentNeutral"
					onclick={() => alert('click center')}
					separator
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					center={{
						type: 'badge-list',
						badgeProps: [
							{ color: 'customFancy', text: 'lorem1' },
							{ color: 'negativePrimary', text: 'lorem2' },
						],
					}}
					right={{
						badgeProps: { color: 'actionPrimary', text: 'lorem' },
						toggleProps: { checked: false, id: 'toggle-2' },
					}}
					verticalPaddings="12px"
					rightPadding="16px"
					leftPadding="16px"
					borderRadius="8px"
					color="neutral"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					left={{
						iconProps: {
							icon: IcMHeartFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
							shape: 'squircle',
						},
						size: 'm',
					}}
					center={{
						type: 'revert',
						size: '400',
					}}
					right={{
						iconProps: {
							icon: IcMChevronRightFilled,
							color: 'graphicActionPrimary',
							bgColor: 'layerFloor1',
						},
						badgeProps: { color: 'customRating', text: 'lorem' },
						indicatorProps: { icon: IcMChildFilled, color: 'accentSecondary' },
					}}
					verticalPaddings="24px"
					rightPadding="24px"
					leftPadding="8px"
					borderRadius="8px"
					color="neutral"
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
			<div class="container">
				<Cell
					center={{
						type: 'accent-icon',
						iconProps: {
							icon: IcMInfoFilled,
							color: 'graphicTertiary',
							onclick: () => alert('click icon'),
						},
					}}
					right={{
						toggleProps: { checked: false, id: 'toggle-1sdf' },
					}}
					verticalPaddings="20px"
					rightPadding="10px"
					leftPadding="16px"
					centralPadding="20px"
					color="neutral"
					separatorAuto
					bind:group={radioGroup}
					onclick={() => alert('click center')}
				>
					<svelte:fragment slot="title">lorem</svelte:fragment>
					<svelte:fragment slot="subtitle">lorem</svelte:fragment>
				</Cell>
			</div>
		</div>
	{/snippet}
</Story>
<Story exportName="events" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="click-example">
			<div>
				<p>Center - link</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							iconProps: {
								icon: IcMChevronRightFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
							},
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							iconProps: {
								icon: IcMChevronRightFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
							},
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							iconProps: {
								icon: IcMChevronRightFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
							},
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							iconProps: {
								icon: IcMChevronRightFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
							},
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - click, Left - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMChildFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							checkboxProps: {
								checked: false,
								id: 'checkbox-888sd',
							},
							size: 'm',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMChildFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							checkboxProps: {
								checked: false,
								id: 'checkbox-2asfxyz',
							},
							size: 'm',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - link, Left - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						onclick={() => alert('click left')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
						}}
						onclick={() => alert('click left')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							checkboxProps: {
								checked: false,
								id: 'checkbox-1hfhj5',
							},
						}}
						onclick={() => alert('click left')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMChildFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							checkboxProps: {
								checked: false,
								id: 'checkbox-2sdfsd1',
							},
							size: 'm',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
						}}
						onclick={() => alert('click left')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - click, Right - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							onclick: () => alert('click right'),
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							onclick: () => alert('click right'),
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							toggleProps: { checked: false, id: 'toggle-1asdf324f' },
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							toggleProps: { checked: false, id: 'toggle-1asdf11111g' },
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - link, Right - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							onclick: () => alert('click right'),
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							onclick: () => alert('click right'),
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							toggleProps: { checked: false, id: 'toggle-1hjfds' },
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							toggleProps: { checked: false, id: 'toggle-1hdssjfds' },
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - link, Left - click, Right - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							onclick: () => alert('click right'),
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							checkboxProps: {
								checked: false,
								id: 'checkbox-1sdfqqq',
							},
						}}
						right={{
							buttonProps: { text: 'lorem', color: 'actionPrimary', id: 'button-1sdf' },
							onclick: () => alert('click right'),
						}}
						onclick={() => alert('click center')}
						color="neutral"
						href="//www.papper.ru"
						target="_blank"
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>

			<div>
				<p>Center - click, Left - click, Right - click</p>
				<div class="container list">
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							iconProps: {
								icon: IcMHeartFilled,
								color: 'graphicActionPrimary',
								bgColor: 'layerFloor1',
								shape: 'squircle',
							},
							size: 'm',
							onclick: () => alert('click left'),
						}}
						right={{
							badgeProps: { color: 'actionPrimary', text: 'lorem' },
							onclick: () => alert('click right'),
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
					<Cell
						center={{
							type: 'default',
							size: '500',
						}}
						left={{
							checkboxProps: {
								checked: false,
								id: 'checkbox-1sdf333a',
							},
						}}
						right={{
							buttonProps: { text: 'lorem', color: 'actionPrimary', id: 'button-1bvbvb' },
							onclick: () => alert('click right'),
						}}
						color="neutral"
						onclick={() => alert('click center')}
						separatorAuto={true}
						verticalPaddings="12px"
						rightPadding="16px"
						leftPadding="16px"
					>
						<svelte:fragment slot="title">lorem</svelte:fragment>
						<svelte:fragment slot="subtitle">lorem</svelte:fragment>
					</Cell>
				</div>
			</div>
		</div>
	{/snippet}
</Story>

<style>
	.main {
		width: 380px;
	}
	.grid {
		display: flex;
		width: 380px;
		flex-direction: column;
		gap: 30px;
	}
	.container {
		width: 380px;
		margin: 16px 0;
	}
	.list {
		display: grid;
		overflow: hidden;
		border-radius: 16px;
		grid-auto-flow: row;
	}
	.usage {
		display: flex;
		flex-wrap: wrap;
		gap: 24px 16px;
	}
	.click-example {
		display: grid;
		column-gap: 32px;
		grid-template-columns: repeat(2, min-content);
	}
</style>
