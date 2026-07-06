import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	const SIZE = 100;
	const RADIUS = SIZE / 2;
	Child($$renderer, {
		a: SIZE,
		b: RADIUS
	});
}
