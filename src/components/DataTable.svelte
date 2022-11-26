<script>
    import {Grid} from 'ag-grid-community';
    import AgGrid from "@budibase/svelte-ag-grid";
    import {onDestroy, onMount} from 'svelte';
    import DataTableToolBar from "./DataTableToolBar.svelte";
    import {activeTable, notificationMsg} from '../stores';


    import 'ag-grid-community/dist/styles/ag-grid.css';
    import 'ag-grid-community/dist/styles/ag-theme-alpine.css';
    import {invoke} from "@tauri-apps/api/tauri";
    import {NOTIFICATION_TYPE_ERROR, PAGINATION_SIZE} from "../constants/constants";

    let domNode;
    let grid;
    let gridApi;
    let gridColumnApi;
    let columnDefs = [];
    let rowDefs = [];
    let rowCount = 0;

    // props
    export let tableData = {};

    onMount(() => {
        console.log("Mounting DataTable")
        let gridOptions = {
            defaultColDef: {
                editable: true,
                sortable: true,
                resizable: true,
                filter: true,
                flex: 1,
                minWidth: 150,
            },
            debounceVerticalScrollbar: true,
            autoSizePadding: 2,
            pagination: true,
            paginationPageSize: PAGINATION_SIZE,
            rowSelection: 'single',
            columnDefs: columnDefs,
            rowData: rowDefs,
            suppressColumnVirtualisation: true,
            suppressRowVirtualisation: true,
            animateRows: false,
            infiniteInitialRowCount: 1,
            suppressPaginationPanel: true
        };

        grid = new Grid(domNode, gridOptions);
        gridOptions.onGridReady = (params) => {
            console.log('OnGridReady: ', params)
            gridApi = params.api;
            gridColumnApi = params.columnApi;
        }
    });

    $: if (gridApi != null && tableData != null) {
        if (tableData.tableName !== "" && tableData.columns.length !== 0) {
            console.log("Refreshing table data")
            columnDefs = []
            rowDefs = []
            rowCount = tableData.rowCount

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
            tableData.columns.forEach(elem => {
                columnDefs.push({
                    field: elem,
                    headerName: elem,
                    sortable: true,
                    comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB
                })
            })

            // set the rows
            tableData.rows.forEach(elem => {
                let singleRowData = {}
                elem.forEach((subElem, index) => {
                    singleRowData[tableData.columns[index]] = subElem
                })
                rowDefs.push(singleRowData)
            })

            gridApi.setColumnDefs(columnDefs);
            gridApi.setRowData(rowDefs);
            gridApi.sizeColumnsToFit();
        }
    }

    let gotoNext = () => {


        gridApi.refreshClientSideRowModel('filter')
        gridApi.paginationGoToNextPage();
    }

    let gotoPrev = () => {
        gridApi.paginationGoToPreviousPage();
    }

    function fetchNextRecordBatch() {
        invoke('fetch_table_data_with_offset', {
            reqPayload: {
                table_name: e,
            },
        }).then((res) => {
            console.log(res);
            let data = res.data;

            activeTable.set({
                tableName: data.table_name,
                columns: data.columns,
                rows: data.rows,
                rowCount: data.row_count,
            })
        }).catch(e => {
            console.log(e)
            notificationMsg.set({
                type: NOTIFICATION_TYPE_ERROR,
                message: e,
            });
        })
    }


    onDestroy(() => {
    });
</script>

<div class="datatable-main-container">
    <div class="datagrid-container">
        <DataTableToolBar
                totalRowCount={rowCount}
                paginationSize={PAGINATION_SIZE}
                gotoNext={gotoNext}
                gotoPrev={gotoPrev}
        />

        <div id="datagrid" bind:this={domNode} class="ag-theme-alpine"></div>
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
        height: 90%;
        position: relative;
        margin: 55px auto 10px auto;
    }

    #datagrid {
        margin-top: 5%;
        height: 90%;
        width: 100%;
        --ag-header-foreground-color: var(--accentColor);
    }
</style>
