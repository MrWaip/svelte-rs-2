<script lang="ts" module>
	import { IcMStatusPointsFilled } from '@papper-ppr-ds/icons';
	import { Badge } from '@papper-ppr-ds/ui-kit/Badge';
	import { Button } from '@papper-ppr-ds/ui-kit/Button';
	import { Image } from '@papper-ppr-ds/ui-kit/Image';
	import { defineMeta } from '@storybook/addon-svelte-csf';

	import { Table } from '$components/Table';
	import { TableBody } from '$components/TableBody';
	import { TableCell } from '$components/TableCell';
	import { TableHeader } from '$components/TableHeader';
	import { TableRow } from '$components/TableRow';
	import type { TableCellSize, TableCellTextSize } from '$types';

	const { Story } = defineMeta({
		title: 'Widgets/Table/Table/TableCell',
		component: TableCell,
		// 👇 Disable auto-generated documentation for this component
		tags: ['!autodocs'],
	});

	type BorderInfo = { withRightBorder: boolean; withBottomBorder: boolean };
	const bordersArray: BorderInfo[] = [
		{ withBottomBorder: true, withRightBorder: false },
		{ withBottomBorder: false, withRightBorder: true },
		{ withBottomBorder: true, withRightBorder: true },
	];
</script>

<Story exportName="sizing" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<div class="tables">
				{#each ['400', '500', '600'] as size}
					<div class="description">
						<span>{size}</span>
						<Table>
							<TableHeader>
								<TableRow>
									<TableCell size={size as TableCellSize}>lorem1</TableCell>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each new Array(3) as _}
									<TableRow>
										<TableCell size={size as TableCellSize}>lorem</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</div>
				{/each}
			</div>
			<div class="tables">
				{#each ['400', '500', '600'] as size}
					<div class="description">
						<Table withBorder>
							<TableHeader>
								<TableRow>
									<TableCell size={size as TableCellSize}>lorem1</TableCell>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each new Array(3) as _}
									<TableRow withSeparator>
										<TableCell size={size as TableCellSize}>lorem</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</div>
				{/each}
			</div>
		</div>
	{/snippet}
</Story>

<Story exportName="textSizing" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<div class="tables">
				{#each ['400', '500'] as textSize}
					<div class="description">
						<span>{textSize}</span>
						<Table>
							<TableHeader>
								<TableRow>
									<TableCell textSize={textSize as TableCellTextSize}>lorem1</TableCell>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each new Array(3) as _}
									<TableRow>
										<TableCell textSize={textSize as TableCellTextSize}>lorem</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</div>
				{/each}
			</div>
			<div class="tables">
				{#each ['400', '500'] as textSize}
					<div class="description">
						<Table withBorder>
							<TableHeader>
								<TableRow>
									<TableCell textSize={textSize as TableCellTextSize}>lorem1</TableCell>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each new Array(3) as _}
									<TableRow withSeparator>
										<TableCell textSize={textSize as TableCellTextSize}>lorem</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</div>
				{/each}
			</div>
		</div>
	{/snippet}
</Story>

<Story exportName="borders" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			{#each bordersArray as { withBottomBorder, withRightBorder }}
				<Table>
					<TableHeader>
						{#each new Array(3) as _, rowIdx}
							<TableRow>
								{#each new Array(3) as __, colIdx}
									<TableCell
										withRightBorder={withRightBorder && colIdx !== 2}
										withBottomBorder={withBottomBorder && rowIdx !== 2}
									>
										lorem
									</TableCell>
								{/each}
							</TableRow>
						{/each}
					</TableHeader>
					<TableBody>
						{#each new Array(3)}
							<TableRow withSeparator={withBottomBorder}>
								{#each new Array(3) as __, colIdx}
									<TableCell withRightBorder={withRightBorder && colIdx !== 2}>lorem</TableCell>
								{/each}
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			{/each}
		</div>
	{/snippet}
</Story>
<Story exportName="outsideBorders" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			{#each bordersArray as { withBottomBorder, withRightBorder }}
				<Table withBorder>
					<TableHeader>
						{#each new Array(3) as _, rowIdx}
							<TableRow>
								{#each new Array(3) as __, colIdx}
									<TableCell
										withRightBorder={withRightBorder && colIdx !== 2}
										withBottomBorder={withBottomBorder && rowIdx !== 2}
									>
										lorem
									</TableCell>
								{/each}
							</TableRow>
						{/each}
					</TableHeader>
					<TableBody>
						{#each new Array(3)}
							<TableRow withSeparator={withBottomBorder}>
								{#each new Array(3) as __, colIdx}
									<TableCell withRightBorder={withRightBorder && colIdx !== 2}>lorem</TableCell>
								{/each}
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			{/each}
		</div>
	{/snippet}
</Story>

<Story exportName="horizontalAlign" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<Table>
				<TableHeader sticky>
					<TableRow>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">lorems</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">lorems</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
				</TableHeader>
				<TableBody>
					<TableRow>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">123456789</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">1234,4</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
					<TableRow>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">1234567</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">1234,4123</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
					<TableRow>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">123</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">1234,412412</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
				</TableBody>
			</Table>
		</div>
	{/snippet}
</Story>
<Story exportName="verticalAlign" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<Table withBorder>
			<TableHeader sticky>
				<TableRow>
					<TableCell>lorem</TableCell>
					<TableCell>lorem</TableCell>
					<TableCell horizontalAlign="right">lorems</TableCell>
					<TableCell>lorem</TableCell>
					<TableCell horizontalAlign="right">lorems</TableCell>
					<TableCell>lorem</TableCell>
					<TableCell>lorem</TableCell>
				</TableRow>
			</TableHeader>
			<TableBody>
				{#each new Array(2) as _, idx}
					{@const verticalAlign = idx ? 'center' : 'top'}
					<TableRow withSeparator>
						<TableCell {verticalAlign}>lorem</TableCell>
						<TableCell {verticalAlign}>
							<Image
								src=example.com
								aspectRatio="1/1"
								containerWidth={48}
								alt="Picture"
								resizing="fixed"
								backgroundColor="graphicNeutral"
								radius={8}
							/>
						</TableCell>
						<TableCell {verticalAlign} horizontalAlign="right">123456789</TableCell>
						<TableCell {verticalAlign}>
							<div>
								<div>loremdiv>
								<div>loremdiv>
								<div>loremdiv>
								<div ppr-typo="tsCompact400Small" ppr-color="text-Secondary">loremdiv>
							</div>
						</TableCell>
						<TableCell {verticalAlign} horizontalAlign="right">
							<div>
								<div class="digits">1234,4</div>
								<div class="digits" ppr-typo="tsCompact400Small" ppr-color="text-Secondary">
									1234,4
								</div>
							</div>
						</TableCell>
						<TableCell {verticalAlign}>lorem</TableCell>
						<TableCell {verticalAlign}>
							<Badge
								size="500"
								color="neutralSecondary"
								text="Action"
								icon={IcMStatusPointsFilled}
								onclick={() => {}}
							/>
						</TableCell>
					</TableRow>
				{/each}
			</TableBody>
		</Table>
	{/snippet}
</Story>

<Story exportName="sticky" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container" style:overflow="auto" style:max-height="400px">
			<Table>
				<TableHeader sticky>
					<TableRow>
						<TableCell left={0} width={100} sticky>lorem</TableCell>
						<TableCell left={100} width={100} sticky withRightBorder>lorem</TableCell>
						<TableCell width={150}>lorem</TableCell>
						<TableCell width={150}>lorem</TableCell>
						<TableCell width={150}>lorem</TableCell>
						<TableCell width={150}>lorem</TableCell>
						<TableCell width={150}>lorem</TableCell>
						<TableCell width={150}>lorem</TableCell>
						<TableCell right={100} width={100} sticky withLeftBorder>lorem</TableCell>
						<TableCell right={0} width={100} sticky>lorem</TableCell>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each new Array(20) as _}
						<TableRow>
							<TableCell hoverable left={0} width={100} sticky>lorem</TableCell>
							<TableCell hoverable left={100} width={100} sticky withRightBorder>lorem</TableCell>
							<TableCell hoverable width={150}>lorem</TableCell>
							<TableCell hoverable width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell hoverable width={150}>lorem</TableCell>
							<TableCell hoverable width={150}>lorem</TableCell>
							<TableCell hoverable right={100} width={100} sticky withLeftBorder>lorem</TableCell>
							<TableCell hoverable right={0} width={100} sticky>lorem</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/snippet}
</Story>

<Story exportName="extraPadding" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<Table withBorder>
				<TableHeader sticky>
					<TableRow>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">lorems</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell horizontalAlign="right">lorems</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each new Array(3) as _}
						<TableRow withSeparator>
							<TableCell extraVerticalPadding={6}>lorem</TableCell>
							<TableCell>
								<Image
									src=example.com
									aspectRatio="1/1"
									containerWidth={48}
									alt="Picture"
									resizing="fixed"
									backgroundColor="graphicNeutral"
									radius={8}
								/>
							</TableCell>
							<TableCell extraVerticalPadding={6} horizontalAlign="right">123456789</TableCell>
							<TableCell extraVerticalPadding={6}>
								<div>
									<div>loremdiv>
									<div ppr-typo="tsCompact400Small" ppr-color="text-Secondary">loremdiv>
								</div>
							</TableCell>
							<TableCell extraVerticalPadding={6} horizontalAlign="right">
								<div>
									<div class="digits">1234,4</div>
									<div class="digits" ppr-typo="tsCompact400Small" ppr-color="text-Secondary">
										1234,4
									</div>
								</div>
							</TableCell>
							<TableCell extraVerticalPadding={6}>lorem</TableCell>
							<TableCell extraVerticalPadding={4}>
								<Badge
									size="500"
									color="neutralSecondary"
									text="Action"
									icon={IcMStatusPointsFilled}
									onclick={() => {}}
								/>
							</TableCell>
							<TableCell>
								<Button size="400" label="Action" color="actionSecondary" />
							</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/snippet}
</Story>

<Story exportName="grouping" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			{#each new Array(2) as _, idx}
				{@const withBorder = Boolean(idx)}
				<Table {withBorder}>
					<TableHeader sticky>
						<TableRow>
							<TableCell colspan={2} withRightBorder withBottomBorder />
							<TableCell colspan={2} withRightBorder withBottomBorder>lorem</TableCell>
							<TableCell colspan={3} withBottomBorder>lorem</TableCell>
						</TableRow>
						<TableRow>
							<TableCell withRightBorder>lorem</TableCell>
							<TableCell withRightBorder>lorem</TableCell>
							<TableCell horizontalAlign="right" withRightBorder>lorems</TableCell>
							<TableCell withRightBorder>lorem</TableCell>
							<TableCell horizontalAlign="right" withRightBorder>lorems</TableCell>
							<TableCell withRightBorder>lorem</TableCell>
							<TableCell>lorem7</TableCell>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each new Array(5) as _, rowInd}
							<TableRow>
								<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
								<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
								<TableCell horizontalAlign="right" withRightBorder>
									lorem.{rowInd + 1}
								</TableCell>
								<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
								<TableCell horizontalAlign="right" withRightBorder>
									lorem.{rowInd + 1}
								</TableCell>
								<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
								<TableCell>lorem.{rowInd + 1}</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			{/each}
		</div>
	{/snippet}
</Story>

<style>
	:global(*) {
		box-sizing: border-box;
	}
	:global(.docs-story) {
		background-color: var(--layerFloor1);
	}
	.container {
		display: flex;
		width: 100%;
		flex-direction: column;
		gap: 32px;
	}
	.tables {
		display: flex;
		overflow: hidden;
		width: 100%;
		height: fit-content;
		gap: 48px;
	}
	.description {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.digits {
		text-align: right;
	}
</style>
