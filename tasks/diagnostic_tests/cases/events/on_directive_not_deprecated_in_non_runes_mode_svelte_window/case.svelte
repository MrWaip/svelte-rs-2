<script lang="ts">
	import { PlayingCard } from 'odd-components';

	type Suite = 'SPADES' | 'HEARTS' | 'DIAMONDS' | 'CLUBS';
	type CardValue = '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | 'J' | 'Q' | 'K' | 'A';
	type Card = {
		suit: Suite;
		value: CardValue;
	};

	const suits: Suite[] = ['CLUBS', 'SPADES', 'HEARTS', 'DIAMONDS'];
	const values: CardValue[] = ['A', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K'];

	const deckMaker = (size: number): Card[] => {
		let deck: Card[] = [];
		let suitIdx, valueIdx;

		for (let i = 0; i < size; i++) {
			suitIdx = Math.floor(Math.random() * suits.length);
			valueIdx = Math.floor(Math.random() * values.length);

			deck.push({ value: values[valueIdx], suit: suits[suitIdx] });
		}

		return deck;
	};

	let my_deck = deckMaker(66);

	const handleScroll = () => {
		const main = document.querySelector('.main')!;
		const cards = document.querySelector('.cards')! as HTMLDivElement;

		const mainRect = main.getBoundingClientRect();
		const offset = window.scrollY - mainRect.top;
		cards.style.transform = `translateY(${-offset * 0.05}px)`;
	};
</script>

<svelte:window on:scroll={handleScroll} />

<div class="main">
	<div class="cover"></div>
	<div class="cards">
		{#each my_deck as card (card)}
			<div class="card-wrapper">
				<PlayingCard {card} />
			</div>
		{/each}
	</div>
</div>
