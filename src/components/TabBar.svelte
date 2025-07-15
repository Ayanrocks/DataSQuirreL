<script lang="ts">
    let tabs = [
        {
            tab_type: "table",
            table_name: "Home",
            schema_name: "public",
            table_rows_count: 100,
            table_rows_selected: 10,
            table_columns: [
                {
                    column_name: "id",
                    column_type: "integer",
                },
                {
                    column_name: "name",
                    column_type: "text",
                }
            ],
            table_rows: [
                {
                    row_id: 1,
                }
            ]
        }
    ]
</script>

<div class="tabbar-container">
    
</div>


<style>
    .tabbar-container {
        position: fixed;
        top: 0;
        display: flex;
        justify-content: space-between;
        flex-direction: row;
        align-items: center;
        background-color: var(--tertiaryColor);
        padding: 10px;
        margin-top: 50px;
        height: 30px;
        width: 100%;
        overflow-x: auto; /* Allow horizontal scrolling for many tabs */
    }

</style>