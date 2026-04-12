import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { beforeUpdate, afterUpdate } from "svelte";
var root = $.from_html(`<p>hooks</p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	beforeUpdate(() => {
		console.log("before");
	});
	afterUpdate(() => {
		console.log("after");
	});
	var p = root();
	$.append($$anchor, p);
	$.pop();
}
