import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const SIZE = 100;
	const RADIUS = SIZE / 2;
	Child($$anchor, {
		a: SIZE,
		b: RADIUS
	});
}
