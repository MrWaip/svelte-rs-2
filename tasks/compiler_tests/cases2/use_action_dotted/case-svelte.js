import * as $ from "svelte/internal/client";
import { actions } from "./actions.js";
var root = $.from_html(`<div>text</div>`);
export default function App($$anchor) {
	var div = root();
	$.action(div, ($$node) => actions.tooltip?.($$node));
	$.append($$anchor, div);
}
