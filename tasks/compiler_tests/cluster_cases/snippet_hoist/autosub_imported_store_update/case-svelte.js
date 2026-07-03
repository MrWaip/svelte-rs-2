import * as $ from "svelte/internal/client";
import { count } from "./store_mod.js";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const foo = ($$anchor) => {
		var button = root();
		$.delegated("click", button, () => $.update_store(count, $count()));
		$.append($$anchor, button);
	};
	foo($$anchor);
	$$cleanup();
}
$.delegate(["click"]);
