<script>
    import {Grid} from 'ag-grid-community';
    import AgGrid from "@budibase/svelte-ag-grid";
    import {onDestroy, onMount} from 'svelte';
    import {activeTable} from '../stores';


    import 'ag-grid-community/dist/styles/ag-grid.css';
    import 'ag-grid-community/dist/styles/ag-theme-material.css';

    let domNode;
    let grid;
    let gridApi;
    let gridColumnApi;
    let columnDefs = [];
    let rowDefs = [];

    let gridOptions = {
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
        paginationPageSize: 20,
        rowSelection: 'single',
        columnDefs: columnDefs,
        rowData: rowDefs,
        suppressColumnVirtualisation: true,
        suppressRowVirtualisation: true,
        animateRows: true
    };


    onMount(() => {
        grid = new Grid(domNode, gridOptions);
        gridOptions.onGridReady = (params) => {
            console.log('OnGridReady: ', params)
            gridApi = params.api;
            gridColumnApi = params.columnApi;
        }
    });

    activeTable.subscribe(val => {
        if (val !== null && val.tableName != "" && val.columns.length !== 0) {
            columnDefs = []
            rowDefs = []
            columnDefs.push({
                headerName: '#',
                valueGetter: 'node.id',
                pinned: 'left',
                lockPinned: 'left',
                sortable: true,
                editable: false,
                cellEditorPopup: false,
                comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB
            });

            // set the columns
            val.columns.forEach(elem => {
                columnDefs.push({
                    field: elem,
                    headerName: elem,
                    sortable: true,
                    comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB
                })
            })

            // set the rows
            val.rows.forEach(elem => {
                let singleRowData = {}
                elem.forEach((subElem, index) => {
                    singleRowData[val.columns[index]] = subElem
                })
                rowDefs.push(singleRowData)
            })

            gridApi.setColumnDefs(columnDefs);
            gridApi.setRowData(rowDefs);
            gridColumnApi.autoSizeAllColumns();
        }

    })

    onDestroy(() => {
        // if (grid) {
        //     grid.destroy();
        // }
    });
</script>

<div class="datatable-main-container">
    <div class="datagrid-container">
        <div id="datagrid" bind:this={domNode} class="ag-theme-material"></div>
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
