import * as $ from "svelte/internal/server";
function pair($$renderer) {
	$$renderer.push(`<div class="after svelte-1hn6tgg">after</div>`);
}
export default function App($$renderer) {
	$$renderer.push(`<span class="before svelte-1hn6tgg">before</span> `);
	pair($$renderer);
	$$renderer.push(`<!----> <div>other</div>`);
}
