<script>
    import {Grid} from 'ag-grid-community';
    import {onDestroy, onMount} from 'svelte';

    import 'ag-grid-community/dist/styles/ag-grid.css';
    import 'ag-grid-community/dist/styles/ag-theme-material.css';

    let domNode;
    let grid;

    var data = {
        columnDefs: [
            {headerName: 'Row ID', valueGetter: 'node.id'},
            {field: 'make'},
            {field: 'model'},
            {field: 'price'},
            {field: 'class'},
            {field: 'category'},
            {field: 'discount'},
        ],
        rowDefs: [
            {
                id: 'c1',
                make: 'Toyota',
                model: 'Celica',
                price: 35000,
                class: 'sports',
                category: "Expensive",
                discount: '25%',
            },
            {
                id: 'c2',
                make: 'Ford',
                model: 'Mondeo',
                price: 32000,
                class: 'muscle',
                category: "Expensive",
                discount: '5%'
            },
            {
                id: 'c8',
                make: 'Porsche',
                model: 'Boxster',
                price: 72000,
                class: 'sports',
                category: "Expensive",
                discount: '2%'
            },
            {
                id: 'c4',
                make: 'BMW',
                model: 'M50',
                price: 60000,
                class: 'sedan',
                category: "Expensive",
                discount: '8%'
            },
            {
                id: 'c14',
                make: 'Aston Martin',
                model: 'DBX',
                price: 190000,
                class: 'sports',
                category: "Expensive",
                discount: '5%'
            },
        ]
    };

    const gridOptions = {
        defaultColDef: {
            editable: true,
            enableRowGroup: true,
            enablePivot: true,
            enableValue: true,
            sortable: true,
            resizable: true,
            filter: true,
            flex: 1,
            minWidth: 100,
        },
        debounceVerticalScrollbar: true,
        autoSizePadding: 4,
        pagination: true,
        paginationPageSize: 10,
        rowSelection: 'single',
        columnDefs: data.columnDefs,
        rowData: data.rowDefs,
        suppressColumnVirtualisation: true,
        suppressRowVirtualisation: true,
    };

    onMount(() => {
        grid = new Grid(domNode, gridOptions);
    });

    onDestroy(() => {
        if (grid) {
            grid.destroy();
        }
    });
</script>

<div class="datatable-main-container">
  <div class="datagrid-container">
    <div id="datagrid" bind:this={domNode} class="ag-theme-material"/>
  </div>
</div>

<style>
    .datatable-main-container {
        margin-top: 10px;
        width: 100%;
        height: 100%;
        padding: 10px;
    }

    .datagrid-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    #datagrid {
        height: 90%;
        width: 100%;
        --ag-header-foreground-color: var(--accentColor);
    }
</style>
