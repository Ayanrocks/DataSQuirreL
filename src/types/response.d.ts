export interface IPCResponse<T> {
    status: number;
    error_code?: string;
    sys_err?: string;
    frontend_msg?: string;
    data?: T;
}

export interface StoredConnection {
    id: string,
    conn_name: string;
    host_name: string;
    database_name: string;
    database_type: string;
    port: number;
    user_name: string;
}