import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div data-tags="card active" class="svelte-v4glr4">class</div> <div data-lang="en-US" class="svelte-v4glr4">lang</div> <div data-url="https://example.com" class="svelte-v4glr4">href</div> <span data-tags="inactive">no class</span> <div data-lang="bengali">no lang</div> <div data-url="http://sample.org">no href</div>`);
}
