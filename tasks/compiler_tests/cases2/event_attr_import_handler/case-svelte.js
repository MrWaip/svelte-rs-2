import * as $ from "svelte/internal/client";
import { handler } from "./module.js";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor) {
	var button = root();
	$.delegated("click", button, function(...$$args) {
		handler?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
