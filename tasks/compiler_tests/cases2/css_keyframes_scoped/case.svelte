<style>
	@keyframes bounce {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	@keyframes -global-slide {
		from { transform: translateX(-100%); }
		to { transform: translateX(0); }
	}

	p {
		animation: bounce 2s ease;
	}

	div {
		animation-name: bounce, slide;
	}
</style>

<p>animated</p>
<div>also animated</div>
