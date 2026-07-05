import * as $ from "svelte/internal/server";
import { count } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$$renderer.push(`<button>set</button> <button>+=</button> <button>??=</button> <button>&amp;&amp;=</button> <button>||=</button> <button>++</button> <button>--pre</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
