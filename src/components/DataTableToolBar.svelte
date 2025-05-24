<script lang="ts">
    import {activeTable} from '../stores';

    let activeTableName = ''

    export let currentPage = 1;
    export let maxPage = 0;
    export let gotoNext;
    export let gotoPrev;

    activeTable.subscribe(value => {
        activeTableName = value.tableName
    })


    let onClickNext = () => {
        if (currentPage === maxPage) {
            return
        }


        // activeTable.update(val => ({currentPage: val + 1}))

        // add logic to fetch more data
        gotoNext()
    }

    let onClickPrev = () => {
        if (currentPage === 1) {
            return
        }

        // activeTable.set({currentPage: currentPage - 1})
        gotoPrev()
    }

</script>

<div class="data-table-toolbar__container">
    <div class="data-table-toolbar-controls__container">
        <div class="data-table-toolbar__controls--left">
            <i class="fa fa-play" aria-hidden="true"></i>
            <h1>{activeTableName}</h1>
        </div>
        <div class="data-table-toolbar__controls--center">
            <h1>world</h1>
        </div>
        <div class="data-table-toolbar__controls--right">
            <div class="controls__right--buttons">
                <button class="controls__right__prev" on:click={onClickPrev} aria-label="Previous Page">
                    <i class="fa fa-arrow-circle-left" aria-hidden="true"></i>
                </button>

                <button class="controls__right__next" on:click={onClickNext} aria-label="Next Page">
                    <i class="fa fa-arrow-circle-right" aria-hidden="true"></i>
                </button>
            </div>
            <div class="page--info">
                <p>Page [{currentPage}] of [{maxPage}]</p>
            </div>
        </div>
    </div>
</div>

<style>
    .fa-arrow-circle-left, .fa-arrow-circle-right {
        color: var(--offWhite);
        transition: 0.3s ease-in-out;
    }

    .data-table-toolbar__container {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 40px;
        background-color: var(--secondaryColor);
        color: var(--offWhite);
        font-size: 12px;
        border-radius: 4px;
    }

    .data-table-toolbar-controls__container {
        display: flex;
        justify-content: space-between;
    }


    .data-table-toolbar__controls--left,
    .data-table-toolbar__controls--center,
    .data-table-toolbar__controls--right {
        width: 33%;
        padding: 5px;
    }

    .data-table-toolbar__controls--right {
        display: flex;
        justify-content: flex-start;
    }

    .controls__right--buttons {
        margin: 5px;
    }

    .controls__right--buttons button {
        background: transparent;
        border: none;
        transform: scale(2);
    }

    .controls__right--buttons button:hover .fa-arrow-circle-left,
    .controls__right--buttons button:hover .fa-arrow-circle-right {
        color: var(--accentColor);
    }

    .page--info {
        margin: 5px;
    }

</style>
