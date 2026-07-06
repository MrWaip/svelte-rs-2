import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	Widget($$renderer, {});
}
