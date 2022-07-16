<script>
    export let loaderActive = false
    export let height = '20px';
    export let width = '20px';
    export let color = '#000';
</script>

{#if loaderActive === true}
    <div class="lds-dual-ring" style="height:{height};--width:{width}; --color:{color}">

    </div>
{/if}

<style>
    .lds-dual-ring {
        display: inline-block;
        width: var(--width);
        height: var(--height);
        margin: 8px;
        border-radius: 50%;
        border: 2px solid var(--color);
        border-color: var(--color) transparent var(--color) transparent;
        animation: lds-dual-ring 1.2s linear infinite;
    }

    @keyframes lds-dual-ring {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

</style>