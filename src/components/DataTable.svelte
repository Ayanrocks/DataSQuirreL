<script>
    import {Grid} from 'ag-grid-community';
    import {onDestroy, onMount} from 'svelte';
    import {activeTable} from '../stores';


    import 'ag-grid-community/dist/styles/ag-grid.css';
    import 'ag-grid-community/dist/styles/ag-theme-material.css';

    let domNode;
    let grid;
    let columnDefs = [];
    let rowDefs = [];

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
        columnDefs: columnDefs,
        rowData: rowDefs,
        suppressColumnVirtualisation: true,
        suppressRowVirtualisation: true,
    };

    onMount(() => {
        new Grid(domNode, gridOptions);
    });

    activeTable.subscribe(val => {
        columnDefs.push({
            headerName: 'Row ID', valueGetter: 'node.id',
        });

        // set the columns
        val.columns.forEach(elem => {
            console.log('COl: ', elem)
            columnDefs.push({
                field: elem
            })
        })

        // set the rows
        val.rows.forEach(elem => {
            let singleRowData = {}
            console.log("Row: ", elem)
            elem.forEach((subElem, index) => {
                singleRowData[val.columns[index]] = subElem
            })
            rowDefs.push(singleRowData)
        })

        gridOptions.api.setColumnDefs(columnDefs);
        gridOptions.api.setRowData(rowDefs);

    })

    onDestroy(() => {
        if (grid) {
            grid.destroy();
        }
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
