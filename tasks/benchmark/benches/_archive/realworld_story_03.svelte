<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const { Story } = defineMeta({
		title: 'Widgets/Calendar',
		tags: ['!autodocs'],
	});
</script>

<script lang="ts">
	import CalendarDay from '../elements/CalendarDay.svelte';
	import CalendarMonth from '../elements/CalendarMonth.svelte';
	import CalendarYear from '../elements/CalendarYear.svelte';
	import CalendarDesktop from '../ui/CalendarDesktop.svelte';

	let states = $state<{
		controlled: Date | undefined;
	}>({
		controlled: new Date(),
	});
</script>

<Story exportName="calendar_desktop_main" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop onSelect={console.log} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_multiple" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop selectionMode="multiple" onSelect={console.log} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_range" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop selectionMode="range" onSelect={console.log} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_controlled" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop
				selected={states.controlled}
				onSelect={(newDate: Date) => {
					console.log(newDate);
					states.controlled = newDate;
				}}
			/>
			<div class="status">
				<p>loremd: {states.controlled?.toLocaleDateString()}</p>
				<button onclick={() => (states.controlled = undefined)}>lorem</button>
				<button
					onclick={() => {
						if (!states.controlled) return;
						const newDate = new Date(states.controlled);
						newDate.setDate(states.controlled.getDate() + 1);
						states.controlled = newDate;
					}}>lorem lorem</button>
			</div>
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_minmax" tags={['!dev']}>
	{#snippet template()}
		{@const today = new Date()}

		<CalendarDesktop
			min={(() => {
				const minDate = new Date();
				minDate.setDate(today.getDate() - 2);
				return minDate;
			})()}
			max={(() => {
				const maxDate = new Date();
				maxDate.setMonth(today.getMonth() + 1);
				return maxDate;
			})()}
			onSelect={console.log}
		/>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_disabled" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop
			isDisabled={(date) => {
				/* lorem4thlorem */
				const isDayDisabled = date.getDate() % 4 === 0;
				/* lorem lorem */
				const isMonthDisabled = [11, 0, 1].includes(date.getMonth());
				/* Year lorem7 */
				const isYearDisabled = date.getFullYear() === 2027;
				return isDayDisabled || isMonthDisabled || isYearDisabled;
			}}
			onSelect={console.log}
		/>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_range_limit" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop selectionMode="range" maxRangeLength={4} onSelect={console.log} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_modeDays" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop viewMode="days" currentDate={new Date()} onSelect={console.log} />
			<CalendarDesktop viewMode="days" currentDate={new Date()} size="400" onSelect={console.log} />
			<CalendarDesktop
				viewMode="days"
				size="400"
				currentDate={new Date()}
				subDayInfo={(date) => date.getDate()}
				onSelect={console.log}
			/>
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_modeMonths" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop viewMode="months" currentDate={new Date()} onSelect={console.log} />
			<CalendarDesktop
				viewMode="months"
				currentDate={new Date()}
				size="400"
				onSelect={console.log}
			/>
			<CalendarDesktop
				viewMode="months"
				currentDate={new Date()}
				size="400"
				subMonthInfo={() => '4000lorem'}
				onSelect={console.log}
			/>
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_modeYears" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop viewMode="years" currentDate={new Date()} onSelect={console.log} />
			<CalendarDesktop
				viewMode="years"
				currentDate={new Date()}
				size="400"
				onSelect={console.log}
			/>
			<CalendarDesktop
				viewMode="years"
				currentDate={new Date()}
				size="400"
				subYearInfo={() => 'lorem'}
				onSelect={console.log}
			/>
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_some_calendars_days" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop numberOfMonths={2} onSelect={console.log} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_additionalContent" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop>
				{#snippet renderRightContent()}
					<div class="additional-content"></div>
				{/snippet}
			</CalendarDesktop>

			<CalendarDesktop>
				{#snippet renderLeftContent()}
					<div class="additional-content"></div>
				{/snippet}
			</CalendarDesktop>
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_weekends" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop isWeekend={(date) => date.getDay() === 0} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_currentDate" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop
				currentDate={(() => {
					const date = new Date();
					date.setDate(new Date().getDate() + 2);
					return date;
				})()}
			/>
			<CalendarDesktop currentDate={null} />
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_outside_days" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop showOutsideDays={false} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_initial_date" tags={['!dev']}>
	{#snippet template()}
		<CalendarDesktop initialDate={new Date(2003, 5, 6)} />
	{/snippet}
</Story>

<Story exportName="calendar_desktop_locale" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop locale="en" />
			<CalendarDesktop sundayFirst />
		</div>
	{/snippet}
</Story>

<Story exportName="calendar_desktop_custom" tags={['!dev']}>
	{#snippet template()}
		<div class="wrapper">
			<CalendarDesktop>
				{#snippet renderDay({ date, selected, hovered })}
					<div class="customDay" class:selected class:hovered>
						{date.getDate()}
					</div>
				{/snippet}
				{#snippet renderDaysGrid({ daysOfView, renderDate })}
					<div class="custom-grid">
						{#each daysOfView as date (date)}
							{@render renderDate({ date })}
						{/each}
					</div>
				{/snippet}
			</CalendarDesktop>

			<CalendarDesktop viewMode="months">
				{#snippet renderMonth(props)}
					<div class="customMonth">
						<CalendarMonth {...props} subInfo={'4000'} />
					</div>
				{/snippet}
			</CalendarDesktop>

			<CalendarDesktop viewMode="years" noViewNavigation>
				{#snippet renderYear(props)}
					<CalendarYear {...props} />
				{/snippet}
			</CalendarDesktop>
		</div>
	{/snippet}
</Story>

<!-- lorem -->

<Story exportName="calendar_desktop_dayVariants" tags={['!dev']}>
	{#snippet template()}
		{@const date = new Date()}
		{@const variants = ['300', '400', '400info']}
		{@const xStates = ['default', 'weekend', 'today', 'outsideMonth', 'disabled', 'hovered']}
		{@const yStates = [
			'default',
			'selected',
			'rangeStart',
			'rangeEnd',
			'inRange',
			'weekend',
			'today',
			'outsideMonth',
			'disabled',
			'hovered',
		]}
		<div class="wrapper">
			{#each variants as variant}
				<div class="states-table" class:size400={variant !== '300'}>
					<!-- loremcloremr -->
					<div class="corner-cell"></div>

					<!-- lorem -->
					{#each xStates as xState}
						<div class="state-header x-axis">{xState}</div>
					{/each}

					<!-- lorem -->
					{#each yStates as yState}
						<div class="state-header y-axis">{yState}</div>

						{#each xStates as xState}
							<div class="day-cell">
								<CalendarDay
									{date}
									weekend={xState === 'weekend' || yState == 'weekend'}
									today={xState === 'today' || yState == 'today'}
									outsideMonth={xState === 'outsideMonth' || yState == 'outsideMonth'}
									disabled={xState === 'disabled' || yState == 'disabled'}
									selected={xState === 'selected' || yState == 'selected'}
									rangeStart={xState === 'rangeStart' || yState == 'rangeStart'}
									rangeEnd={xState === 'rangeEnd' || yState == 'rangeEnd'}
									inRange={xState === 'inRange' || yState == 'inRange'}
									subInfo={variant === '400info' ? '4000lorem' : undefined}
									hovered={xState === 'hovered' || yState == 'hovered'}
									size={variant.includes('400') ? '400' : '300'}
								/>
							</div>
						{/each}
					{/each}
				</div>
			{/each}
		</div>
	{/snippet}
</Story>

<style>
	.status {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}
	.customDay {
		position: relative;
		display: flex;
		width: 48px;
		height: 48px;
		box-sizing: border-box;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
		cursor: pointer;
	}
	.customDay.selected::after {
		position: absolute;
		top: 28px;
		left: 28px;
		width: 18px;
		height: 18px;
		box-sizing: content-box;
		border: 3px solid white;
		border-radius: 100%;
		background-color: #005bff;
		content: '';
	}
	.customDay.selected {
		border: 2px solid #005bff;
		background-color: #0096ff14;
	}
	.customDay:hover {
		background-color: #0096ff14;
	}
	.customMonth > :global(.month) {
		height: 60px;
	}

	.states-table {
		--size-cell: 32px;
		display: grid;
		width: fit-content;
		height: fit-content;
		gap: 12px;
		grid-template-columns: 120px repeat(6, var(--size-cell));
		place-items: center;
	}
	.states-table.size400 {
		--size-cell: 48px;
	}

	.corner-cell {
		height: 100%;
		grid-column: 1;
		grid-row: 1;
	}

	.state-header {
		width: 100%;
		box-sizing: border-box;
		padding: 8px;
		border-radius: 4px;
		background: var(--background-secondary);
		font-size: 12px;
		font-weight: 500;
		text-align: center;
	}

	.state-header.x-axis {
		transform: rotate(-45deg);
	}

	.state-header.y-axis {
		text-align: end;
	}

	.day-cell {
		display: flex;
		width: var(--size-cell);
		height: var(--size-cell);
		align-items: center;
		justify-content: center;
		border: 1px dashed red;
		background-color: var(--layer-Floor2);
	}

	.wrapper {
		display: flex;
		gap: 20px;
	}

	.additional-content {
		display: flex;
		width: 200px;
		height: 100%;
		flex-direction: column;
		background-color: #f6c7feb2;
		gap: 8px;
	}

	.custom-grid {
		display: grid;
		gap: 8px;
		grid-template-columns: repeat(7, 1fr);
	}
</style>
