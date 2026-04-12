import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { beforeUpdate as before, afterUpdate as after } from "svelte";
var root = $.from_html(`<p>hooks</p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	before(() => {
		console.log("before");
	});
	after(() => {
		console.log("after");
	});
	var p = root();
	$.append($$anchor, p);
	$.pop();
}
