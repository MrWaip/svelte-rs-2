import * as $ from "svelte/internal/client";
import tooltip from "./tooltip.js";
export default function App($$anchor) {
	$.action($.window, ($$node) => tooltip?.($$node));
}
