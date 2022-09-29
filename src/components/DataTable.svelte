<script>
    import {Grid} from 'ag-grid-community';
    import AgGrid from "@budibase/svelte-ag-grid";
    import {onDestroy, onMount} from 'svelte';
    import DataTableToolBar from "./DataTableToolBar.svelte";
    import {activeTable, notificationMsg} from '../stores';


    import 'ag-grid-community/dist/styles/ag-grid.css';
    import 'ag-grid-community/dist/styles/ag-theme-material.css';
    import {invoke} from "@tauri-apps/api/tauri";
    import {NOTIFICATION_TYPE_ERROR} from "../constants/constants";

    let domNode;
    let grid;
    let gridApi;
    let gridColumnApi;
    let columnDefs = [];
    let rowDefs = [];
    let rowCount = 0;
    let paginationSize = 20

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
                minWidth: 100,
            },
            debounceVerticalScrollbar: true,
            autoSizePadding: 4,
            pagination: true,
            paginationPageSize: paginationSize,
            rowSelection: 'single',
            columnDefs: columnDefs,
            rowData: rowDefs,
            suppressColumnVirtualisation: true,
            suppressRowVirtualisation: true,
            animateRows: true,
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
            gridColumnApi.autoSizeAllColumns();
        }
    }

    let gotoNext = () => {
        gridApi.paginationGoToNextPage();
        let currentPage = gridApi.paginationGetCurrentPage() + 1;
        let totalRowFetched = gridApi.paginationGetRowCount();
        console.log("Paging size: ", currentPage, totalRowFetched, paginationSize)

        // 40/20 - 2 = 2-2 = 0 => 1 > 0 && 1 && 152/20
        if ((currentPage > ((totalRowFetched / paginationSize) - 2)) && totalRowFetched !== rowCount) {
            // fetch more rows
            console.log("Fetch More rows")
            // Invoke the command
            invoke('fetch_table_data_with_offset', {
                reqPayload: {
                    table_name: tableData.tableName,
                    offset: 40 * (currentPage - 1),
                },
            }).then((res) => {
                console.log(res);
                let data = res.data;

                // activeTable.set({
                //     tableName: data.table_type,
                //     columns: data.columns,
                //     rows: data.rows,
                //     rowCount: data.row_count,
                // })

                tableData.rows = [...tableData.rows, ...data.rows]
                console.log("Data: ", tableData.rows)
            }).catch(e => {
                console.log(e)
                notificationMsg.set({
                    type: NOTIFICATION_TYPE_ERROR,
                    message: e,
                });
            })
        }

    }

    let gotoPrev = () => {
        gridApi.paginationGoToPreviousPage();
    }


    onDestroy(() => {
        // if (grid) {
        //     grid.destroy();
        // }
    });
</script>

<div class="datatable-main-container">
    <div class="datagrid-container">
        <DataTableToolBar
                totalRowCount={rowCount}
                paginationSize={paginationSize}
                gotoNext={gotoNext}
                gotoPrev={gotoPrev}
        />

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
