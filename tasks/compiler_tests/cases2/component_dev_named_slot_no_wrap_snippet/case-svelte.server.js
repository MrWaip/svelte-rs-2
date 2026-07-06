import * as $ from "svelte/internal/server";
import Card from "./Card.svelte";
export default function App($$renderer) {
	Card($$renderer, { $$slots: { footer: ($$renderer) => {
		{
			$$renderer.push(`footer`);
		}
	} } });
}
