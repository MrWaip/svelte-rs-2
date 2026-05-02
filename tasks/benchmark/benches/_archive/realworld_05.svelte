<script lang="ts">
	import { YEARS_PER_CALENDAR_COUNT } from '../consts';
	import CalendarDay from '../elements/CalendarDay.svelte';
	import CalendarDaysGrid from '../elements/CalendarDaysGrid.svelte';
	import CalendarDesktopContentWrapper from '../elements/CalendarDesktopContentWrapper.svelte';
	import CalendarDesktopHeader from '../elements/CalendarDesktopHeader.svelte';
	import CalendarDesktopHeaderAction from '../elements/CalendarDesktopHeaderAction.svelte';
	import CalendarDesktopWrapper from '../elements/CalendarDesktopWrapper.svelte';
	import CalendarMonth from '../elements/CalendarMonth.svelte';
	import CalendarMonthsGrid from '../elements/CalendarMonthsGrid.svelte';
	import CalendarYear from '../elements/CalendarYear.svelte';
	import CalendarYearsGrid from '../elements/CalendarYearsGrid.svelte';
	import {
		addMonths,
		addYears,
		getFirstCalendarDate,
		getFirstYearsViewDate,
		getLastYearsViewDate,
		getRangeFlags,
		handleSelect,
		isBefore,
		isDateSelected,
		isDayOutOfRange,
		isMonthOutOfRange,
		isNextViewUnavailable,
		isPrevViewUnavailable,
		isSameDay,
		isSameMonth,
		isSameYear,
		isWeekend as isWeekendDefault,
		isYearOutOfRange,
	} from '../helpers/calendar.helpers';
	import { formatMonth, formatYear } from '../helpers/formatDates';
	import type { Nullable } from '../types';
	import type {
		CalendarDayProps,
		CalendarDesktopProps,
		CalendarMonthProps,
		CalendarRange,
		CalendarViewMode,
		CalendarYearProps,
	} from '../types';

	let {
		selected = $bindable(),
		onSelect,
		selectionMode = 'single',

		max: maxDate,
		min: minDate,
		viewMode: viewModeGlobal = 'days',
		showOutsideDays = true,
		currentDate = new Date(),
		numberOfMonths = 1,
		noViewNavigation = false,
		sundayFirst = false,
		initialDate,
		size = '300',
		locale = 'ru',
		maxRangeLength,
		dataTestid = 'ppr-test-id-calendar',

		subDayInfo,
		subMonthInfo,
		subYearInfo,

		isDisabled,
		isWeekend,

		renderLeftContent,
		renderRightContent,
		renderDay,
		renderMonth,
		renderYear,
		renderDaysGrid,
		renderMonthsGrid,
		renderYearsGrid,
		renderCalendarWrapper,
		renderContentWrapper,
	}: CalendarDesktopProps = $props();

	/* Lorem ipsum dolor sit amet consectetur */
	const firstCalendarDate = getFirstCalendarDate(initialDate, selected, selectionMode);

	/* Lorem ipsum dolor sit amet, consectetur adipiscing elit, + 1 dolor */
	let localCalendars = $state(
		Array.from({ length: numberOfMonths }).map((_, i) => addMonths(firstCalendarDate, i))
	);

	/* Lorem ipsum dolor i sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt */
	let selectedView =
		$state<Nullable<{ index: number; view: CalendarViewMode; date: Date; initialDate: Date }>>(
			null
		);

	let hoveredDate = $state<Nullable<Date>>(null);

	function openView(viewToOpen: CalendarViewMode, calendarIndex: number, calendarDate: Date) {
		selectedView = {
			index: calendarIndex,
			date: calendarDate,
			view: viewToOpen,
			initialDate: calendarDate,
		};
	}

	function commitSelectedView(date: Date, viewIndex: number) {
		localCalendars = localCalendars.map((_, index) => {
			if (index === viewIndex) return date;
			return addMonths(date, index - viewIndex);
		});
		selectedView = null;
	}

	function selectDate(date: Date) {
		const newState = handleSelect({ mode: selectionMode, current: selected, date });
		selected = newState;
		onSelect?.(newState);
	}

	function handleDayClick(state: CalendarDayProps, viewDate: Date) {
		if (state.disabled) return;

		if (
			state.outsideMonth &&
			(selectionMode === 'single' ||
				(selectionMode === 'multiple' && !(selected as Date[]).length) ||
				(selectionMode === 'range' &&
					(selected as CalendarRange).start &&
					(selected as CalendarRange).end))
		) {
			if (isBefore(state.date, viewDate)) {
				goPrevDaysView();
			} else {
				goNextDaysView();
			}
		}

		selectDate(state.date);
	}

	function handleMonthClick(state: CalendarMonthProps, viewIndex: number) {
		if (state.disabled) return;

		if (selectedView) {
			return commitSelectedView(state.date, viewIndex);
		}

		selectDate(state.date);
	}

	function handleYearClick(state: CalendarYearProps, viewIndex: number) {
		if (state.disabled) return;
		if (selectedView) {
			return commitSelectedView(state.date, viewIndex);
		}
		selectDate(state.date);
	}

	function handleKeyDownWrapper(callBack: () => void) {
		return (e: KeyboardEvent) => {
			if (e.key == 'Enter' || e.key == ' ') {
				e.preventDefault();
				callBack();
			}
		};
	}

	function getDaysViewLabels(date: Date): [monthLabel: string, yearLabel: string] {
		const monthLabel = formatMonth(date, locale);
		const yearLabel = formatYear(date, locale);
		return [monthLabel[0].toUpperCase() + monthLabel.slice(1), yearLabel];
	}

	function getYearsViewLabel(date: Date) {
		const firstYearDate = getFirstYearsViewDate(date);
		const lastYearDate = getLastYearsViewDate(date);
		const start = formatYear(firstYearDate, locale);
		const end = formatYear(lastYearDate, locale);
		return `${start} – ${end}`;
	}

	function goNextDaysView() {
		localCalendars = localCalendars.map((date) => addMonths(date, 1));
	}
	function goPrevDaysView() {
		localCalendars = localCalendars.map((date) => addMonths(date, -1));
	}
	function goNextMonthsView(i: number) {
		if (selectedView?.index === i) {
			selectedView.date = addYears(selectedView.date, 1);
			return;
		}
		localCalendars = localCalendars.map((date) => addYears(date, 1));
	}
	function goPrevMonthsView(i: number) {
		if (selectedView?.index === i) {
			selectedView.date = addYears(selectedView.date, -1);
			return;
		}
		localCalendars = localCalendars.map((date) => addYears(date, -1));
	}
	function goNextYearsView(index: number) {
		if (selectedView?.index === index) {
			selectedView.date = addYears(selectedView.date, YEARS_PER_CALENDAR_COUNT);
			return;
		}
		localCalendars = localCalendars.map((value) => addYears(value, YEARS_PER_CALENDAR_COUNT));
	}
	const goPrevYearsView = (index: number) => {
		if (selectedView?.index === index) {
			selectedView.date = addYears(selectedView.date, -YEARS_PER_CALENDAR_COUNT);
			return;
		}
		localCalendars = localCalendars.map((value) => addYears(value, -YEARS_PER_CALENDAR_COUNT));
	};

	/* states */

	function dayState(date: Date, viewDate: Date): CalendarDayProps {
		const outsideMonth = !isSameMonth(date, viewDate);
		const hovered = !!hoveredDate && isSameDay(date, hoveredDate);
		const today = !!currentDate && isSameDay(date, currentDate);
		const weekend = isWeekend ? !!isWeekend(date) : isWeekendDefault(date);
		const isSelected = isDateSelected(date, selected, 'days', selectionMode);
		const { inRange, rangeStart, rangeEnd, isDisabledByMaxRange } = getRangeFlags(
			date,
			selected,
			hoveredDate,
			maxRangeLength
		);
		const disabled =
			isDayOutOfRange(date, minDate, maxDate) || isDisabled?.(date) || isDisabledByMaxRange;

		return {
			date,
			today,
			disabled,
			selected: isSelected,
			weekend,
			outsideMonth,
			inRange,
			rangeStart,
			rangeEnd,
			hovered,
			subInfo: subDayInfo?.(date),
			size,
		};
	}

	function monthState(date: Date): CalendarMonthProps {
		const disabled = isMonthOutOfRange(date, minDate, maxDate) || isDisabled?.(date);
		const today = !!currentDate && isSameMonth(date, currentDate);
		const isSelected = isDateSelected(date, selected, 'months', selectionMode);
		return {
			date,
			today,
			disabled,
			selected: isSelected,
			subInfo: subMonthInfo?.(date),
			locale,
			size,
		};
	}

	function yearState(date: Date): CalendarYearProps {
		const disabled = isYearOutOfRange(date, minDate, maxDate) || isDisabled?.(date);
		const today = !!currentDate && isSameYear(date, currentDate);
		const isSelected = isDateSelected(date, selected, 'years', selectionMode);
		return {
			date,
			today,
			disabled,
			selected: isSelected,
			subInfo: subYearInfo?.(date),
			locale,
			size,
		};
	}
</script>

<CalendarDesktopWrapper
	{renderLeftContent}
	{renderRightContent}
	{renderCalendarWrapper}
	{dataTestid}
>
	{#each localCalendars as localCalendarDate, i}
		{@const daysViewActive = selectedView?.index !== i && viewModeGlobal === 'days'}
		{@const monthsViewActive =
			(selectedView?.view === 'months' && selectedView.index === i) ||
			(viewModeGlobal === 'months' && selectedView?.index !== i)}
		{@const yearsViewActive =
			(selectedView?.view === 'years' && selectedView.index === i) || viewModeGlobal === 'years'}

		{#if daysViewActive}
			{@const labels = getDaysViewLabels(localCalendarDate)}
			{@const isViewDisabled = !!selectedView}
			{@const isGoPrevBtnVisible = selectedView?.index === i || i === 0}
			{@const isGoNextBtnVisible = selectedView?.index === i || i === numberOfMonths - 1}

			<CalendarDesktopContentWrapper {renderContentWrapper}>
				{#if !noViewNavigation}
					<CalendarDesktopHeader
						onGoPrev={isGoPrevBtnVisible ? goPrevDaysView : undefined}
						onGoNext={isGoNextBtnVisible ? goNextDaysView : undefined}
						goPrevDisabled={isViewDisabled ||
							isPrevViewUnavailable(localCalendarDate, 'days', minDate)}
						goNextDisabled={isViewDisabled ||
							isNextViewUnavailable(localCalendarDate, 'days', maxDate)}
					>
						<CalendarDesktopHeaderAction
							label={labels[0]}
							onClick={() => openView('months', i, localCalendarDate)}
							disabled={isViewDisabled}
						/>
						<CalendarDesktopHeaderAction
							label={labels[1]}
							onClick={() => openView('years', i, localCalendarDate)}
							disabled={isViewDisabled}
						/>
					</CalendarDesktopHeader>
				{/if}
				<CalendarDaysGrid
					viewDate={localCalendarDate}
					{locale}
					{sundayFirst}
					mobile={false}
					{renderDaysGrid}
				>
					{#snippet renderDate({ date })}
						{@const _state = dayState(date, localCalendarDate)}
						{@const state = { ..._state, disabled: _state.disabled || isViewDisabled }}
						{@const selectHandler = () => handleDayClick(state, localCalendarDate)}
						<div
							role="button"
							aria-disabled={state.disabled}
							onclick={selectHandler}
							onkeydown={handleKeyDownWrapper(selectHandler)}
							onmouseenter={() => (hoveredDate = date)}
							onmouseleave={() => (hoveredDate = null)}
							tabindex={-1}
							data-testid="ppr-test-id-calendar-desktop-day-btn"
						>
							{#if renderDay && !(state.outsideMonth && !showOutsideDays)}
								{@render renderDay(state)}
							{:else if !(state.outsideMonth && !showOutsideDays)}
								<CalendarDay {...state} />
							{/if}
						</div>
					{/snippet}
				</CalendarDaysGrid>
			</CalendarDesktopContentWrapper>
		{:else if monthsViewActive}
			{@const viewDate = selectedView?.index === i ? selectedView.date : localCalendarDate}
			{@const isViewDisabled = selectedView?.view === 'years'}
			{@const isGoPrevBtnVisible = selectedView?.index === i || i === 0}
			{@const isGoNextBtnVisible = selectedView?.index === i || i === numberOfMonths - 1}
			{@const openYearsViewHandler = !isViewDisabled
				? () => openView('years', i, viewDate)
				: undefined}
			<CalendarDesktopContentWrapper {renderContentWrapper}>
				{#if !noViewNavigation}
					<CalendarDesktopHeader
						onGoPrev={isGoPrevBtnVisible ? () => goPrevMonthsView(i) : undefined}
						onGoNext={isGoNextBtnVisible ? () => goNextMonthsView(i) : undefined}
						goPrevDisabled={isViewDisabled || isPrevViewUnavailable(viewDate, 'months', minDate)}
						goNextDisabled={isViewDisabled || isNextViewUnavailable(viewDate, 'months', maxDate)}
					>
						<CalendarDesktopHeaderAction
							label={viewDate.getFullYear().toString()}
							onClick={openYearsViewHandler}
						/>
					</CalendarDesktopHeader>
				{/if}
				<CalendarMonthsGrid {viewDate} {renderMonthsGrid}>
					{#snippet renderDate({ date })}
						{@const _state = monthState(date)}
						{@const state = { ..._state, disabled: _state.disabled || isViewDisabled }}
						{@const selectHandler = () => handleMonthClick(state, i)}
						<div
							role="button"
							aria-disabled={state.disabled}
							onclick={selectHandler}
							onkeydown={handleKeyDownWrapper(selectHandler)}
							tabindex={-1}
							data-testid="ppr-test-id-calendar-desktop-month-btn"
						>
							{#if renderMonth}
								{@render renderMonth(state)}
							{:else}
								<CalendarMonth {...state} />
							{/if}
						</div>
					{/snippet}
				</CalendarMonthsGrid>
			</CalendarDesktopContentWrapper>
		{:else if yearsViewActive}
			{@const viewDate = selectedView?.index === i ? selectedView.date : localCalendarDate}
			{@const isGoPrevBtnVisible = selectedView?.index === i || i === 0}
			{@const isGoNextBtnVisible = selectedView?.index === i || i === numberOfMonths - 1}

			<CalendarDesktopContentWrapper {renderContentWrapper}>
				{#if !noViewNavigation}
					<CalendarDesktopHeader
						onGoPrev={isGoPrevBtnVisible ? () => goPrevYearsView(i) : undefined}
						onGoNext={isGoNextBtnVisible ? () => goNextYearsView(i) : undefined}
						goPrevDisabled={isPrevViewUnavailable(viewDate, 'years', minDate)}
						goNextDisabled={isNextViewUnavailable(viewDate, 'years', maxDate)}
					>
						<CalendarDesktopHeaderAction label={getYearsViewLabel(viewDate)} />
					</CalendarDesktopHeader>
				{/if}
				<CalendarYearsGrid {viewDate} {renderYearsGrid}>
					{#snippet renderDate({ date })}
						{@const state = yearState(date)}
						{@const selectHandler = () => handleYearClick(state, i)}
						<div
							role="button"
							aria-disabled={state.disabled}
							onclick={selectHandler}
							onkeydown={handleKeyDownWrapper(selectHandler)}
							tabindex={-1}
							data-testid="ppr-test-id-calendar-desktop-year-btn"
						>
							{#if renderYear}
								{@render renderYear(state)}
							{:else}
								<CalendarYear {...state} />
							{/if}
						</div>
					{/snippet}
				</CalendarYearsGrid>
			</CalendarDesktopContentWrapper>
		{/if}
	{/each}
</CalendarDesktopWrapper>

<!--
  @component
  ## CalendarDesktop


  ### Usage:
      <CalendarDesktop
        selectionMode="multiple"
        min={new Date(2025, 0, 1)}
        max={new Date(2026, 11, 31)}
        numberOfMonths={2}
        sundayFirst
      />
-->
