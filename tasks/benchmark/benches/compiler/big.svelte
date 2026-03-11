<script>
    import { onMount } from "svelte";

    let {
        title = "Default Title",
        count = 0,
        items = [],
        config = $bindable({}),
        multiplier = 2,
        ...rest
    } = $props();

    let state = $state("");
    let counter = $state(0);

    counter = 10;

    let doubled = $derived(count * multiplier);

    $effect(() => {
        console.log("Title:", title, "Count:", count);
    });
</script>

<div>
    Chunk 0: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 0.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-0">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 1: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 1.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 1.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 1.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-1">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 2: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 2.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 2.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 2.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-2">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 3: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 3.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 3.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 3.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-3">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 4: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 4.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 4.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 4.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-4">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 5: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 5.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 5.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 5.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-5">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 6: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 6.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 6.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 6.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-6">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 7: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 7.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 7.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 7.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-7">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 8: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 8.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 8.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 8.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-8">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 9: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 9.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 9.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 9.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-9">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 10: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 10.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 10.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 10.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-10">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 11: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 11.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 11.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 11.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-11">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 12: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 12.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 12.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 12.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-12">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 13: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 13.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 13.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 13.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-13">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 14: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 14.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 14.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 14.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-14">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 15: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 15.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 15.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 15.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-15">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 16: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 16.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 16.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 16.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-16">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 17: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 17.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 17.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 17.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-17">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 18: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 18.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 18.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 18.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-18">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 19: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 19.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 19.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 19.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-19">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 20: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 20.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 20.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 20.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-20">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 21: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 21.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 21.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 21.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-21">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 22: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 22.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 22.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 22.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-22">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 23: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 23.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 23.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 23.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-23">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 24: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 24.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 24.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 24.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-24">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 25: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 25.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 25.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 25.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-25">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 26: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 26.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 26.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 26.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-26">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 27: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 27.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 27.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 27.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-27">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 28: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 28.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 28.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 28.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-28">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 29: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 29.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 29.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 29.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-29">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 30: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 30.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 30.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 30.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-30">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 31: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 31.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 31.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 31.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-31">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 32: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 32.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 32.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 32.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-32">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 33: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 33.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 33.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 33.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-33">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 34: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 34.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 34.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 34.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-34">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 35: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 35.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 35.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 35.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-35">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 36: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 36.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 36.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 36.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-36">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 37: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 37.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 37.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 37.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-37">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 38: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 38.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 38.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 38.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-38">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 39: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 39.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 39.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 39.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-39">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 40: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 40.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 40.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 40.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-40">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 41: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 41.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 41.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 41.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-41">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 42: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 42.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 42.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 42.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-42">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 43: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 43.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 43.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 43.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-43">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 44: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 44.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 44.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 44.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-44">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 45: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 45.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 45.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 45.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-45">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 46: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 46.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 46.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 46.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-46">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 47: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 47.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 47.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 47.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-47">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 48: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 48.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 48.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 48.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-48">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

<div>
    Chunk 49: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 49.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 49.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 49.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-49">{item}</p>
    {/each}

    <input bind:value={state} />
</div>

