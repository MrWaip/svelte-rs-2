import * as $ from "svelte/internal/client";
import { api } from "./api.js";
var root = $.from_html(`<button>click</button>`);
export default function App($$anchor) {
	function doSomething() {
		api.call();
	}
	var button = root();
	$.delegated("click", button, doSomething);
	$.append($$anchor, button);
}
$.delegate(["click"]);
