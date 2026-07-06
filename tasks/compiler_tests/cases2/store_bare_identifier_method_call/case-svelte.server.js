import * as $ from "svelte/internal/server";
import { timerStore } from "./store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let timerId = void 0;
		function clear() {
			if (timerId) timerStore.clearTimer(timerId);
		}
		function start() {
			timerId = timerStore.createTimer(60);
		}
		$$renderer.push(`<button>start</button> <button>clear</button> `);
		if (timerId && $.store_get($$store_subs ??= {}, "$timerStore", timerStore)[timerId]) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`active`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
