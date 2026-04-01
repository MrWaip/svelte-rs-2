import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor) {
	function done() {}
	Widget($$anchor, {});
}
