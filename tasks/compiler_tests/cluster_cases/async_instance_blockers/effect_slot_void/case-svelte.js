import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	$.user_effect(() => {
		console.log("before");
	});
	var $$promises = $.run([() => 1, () => void $.user_effect(() => {
		console.log("after");
	})]);
	$.pop();
}
