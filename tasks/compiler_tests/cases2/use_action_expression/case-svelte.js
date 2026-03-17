import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.from_html(`<div>text</div>`);
export default function App($$anchor) {
	let config = { text: "hello" };
	var div = root();
	$.action(div, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => config);
	$.append($$anchor, div);
}
