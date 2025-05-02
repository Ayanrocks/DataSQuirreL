<script>
    import {invoke} from '@tauri-apps/api/core';
    import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
    import {onDestroy, onMount} from 'svelte';
    import Sidebar from '../components/Sidebar.svelte';
    import DataTable from '../components/DataTable.svelte';
    import {notificationMsg, tableNames, windowWidth, windowHeight, activeTable} from '../stores';
    import {
        NOTIFICATION_TYPE_ERROR,
        BORDER_SIZE,
        MAX_RESIZE_EXPANDABLE_SIZE,
        MIN_RESIZE_EXPANDABLE_SIZE,
    } from '../constants/constants';
const appWindow = getCurrentWebviewWindow()

    const registerFocus = useFocus();

    // on mousedown for the draggable

    let m_pos;

    /// to resize the window on drag
    function resize(e) {
        const dx = e.x - m_pos;
        m_pos = e.x;
        const leftSidebarContainer = document.getElementById('left-sidebar-container');
        const rightMainContainer = document.getElementById('right-main-content');
        let computedWidth = parseInt(getComputedStyle(leftSidebarContainer, '').width) + dx;
        let computedWidthInPx = computedWidth + 'px'

        if (computedWidth <= MAX_RESIZE_EXPANDABLE_SIZE && computedWidth >= MIN_RESIZE_EXPANDABLE_SIZE) {
            leftSidebarContainer.style.width = computedWidthInPx;
            rightMainContainer.style.width = (windowWidth - computedWidth) + 'px'
            rightMainContainer.style.marginLeft = computedWidthInPx;
        }
    }

    /// to resize the sidebar
    function resizeSideBar(e) {
        e = e.detail.event;
        if (e.offsetX < BORDER_SIZE) {
            m_pos = e.x;
            document.addEventListener('mousemove', resize, false);
        }
    }

    /// adding mousemove and mouseup event listeners
    document.addEventListener(
        'mouseup',
        function () {
            document.removeEventListener('mousemove', resize, false);
        },
        false
    );


    var unlisten;
    let activeTableData = {}

    onDestroy(() => {
        unlisten();
    })

    activeTable.subscribe(val => {
        activeTableData = val
    })

    onMount(() => {
        // on change of width, check and set the width of the main and sidebar content
        windowWidth.subscribe(val => {
            // set initial width of sidebar and main content area
            const leftSidebarContainer = document.getElementById('left-sidebar-container');
            const rightMainContainer = document.getElementById('right-main-content');
            let computedWidth = parseInt(getComputedStyle(leftSidebarContainer, '').width);
            let computedWidthInPx = computedWidth + 'px'

            leftSidebarContainer.style.width = computedWidthInPx;
            rightMainContainer.style.width = (val - computedWidth) + 'px'
            rightMainContainer.style.marginLeft = computedWidthInPx;
        })

        unlisten = appWindow.onResized(async () => {
            const factor = await appWindow.scaleFactor();
            const position = await appWindow.innerSize();
            let logicalSize = position.toLogical(factor);

            windowWidth.set(logicalSize.width);
            windowHeight.set(logicalSize.height);
        });

        // setting the current window height
        appWindow.innerSize().then(async w => {
            const factor = await appWindow.scaleFactor();
            let logicalSize = w.toLogical(factor);

            windowWidth.set(logicalSize.width);
            windowHeight.set(logicalSize.height);
        })


        // fetch tables on load
        invoke('fetch_tables')
            .then((res) => {
                if (res.error_code) {
                    notificationMsg.set({
                        type: NOTIFICATION_TYPE_ERROR,
                        message: res.frontend_msg
                    });
                    return;
                }

                if (res.data) {
                    if (res.data.rows.length > 0) {
                        let tablesResult = [];
                        let entityName = '';
                        for (const i of res.data.rows[0]) {
                            entityName = i.table_catalog;
                            tablesResult.push(i.table_name);
                        }

                        // sort tablenames

                        tablesResult = tablesResult.sort((a, b) => a > b);

                        tableNames.set({
                            tableName: entityName,
                            tables: tablesResult,
                        });
                    }
                }
            })
            .catch((e) => {
                console.log(e);
                notificationMsg.set({
                    type: NOTIFICATION_TYPE_ERROR,
                    message: 'Something went wrong. Check console for more information',
                });
            });
    });
</script>

<div class="main-container">
    <div class="columns split-view-container" id="left-sidebar-container">
        <Sidebar on:resizing={resizeSideBar}/>
    </div>
    <div class="columns split-main-content" id="right-main-content">
        {#if activeTableData.tableName !== ""}
            <DataTable/>
        {/if}
    </div>
</div>

<style>
    .main-container {
        height: 102vh;
        width: 100vw;
        background-color: var(--offWhite);
        color: var(--accentColor);
        display: flex;
        justify-content: flex-start;
    }

    .split-view-container {
        position: absolute;
        background-color: var(--accentColor);
        height: 102%;
        width: 24%;
        min-width: 250px;
        max-width: 600px;
    }

    .split-main-content {
        margin-left: clamp(250px, 24%, 600px);
    }
</style>
