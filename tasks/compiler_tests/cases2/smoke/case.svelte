<h1>
    <span>Entities {logged_in}</span>
    <button>+</button>

    some long text
</h1>

<noscript> any content </noscript>

{#if !loading}
    <div>
        {#if featureA}
            <div>
                <div>
                    {user_name}
                </div>

                <button>{counter}</button>
            </div>
        {:else if featureB}
            <div>
                <p>Lorem</p>
            </div>
        {:else}
            <h2>
                Old UI
            </h2>
        {/if}
    </div>
{:else}
    <div>
        Spinner {percent}
    </div>
{/if}
