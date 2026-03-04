import type { IDbTypeMapper } from "../types/dbTypeMapper";

export class PostgresMapper implements IDbTypeMapper {
    sqlToJsType(sqlType: string): string {
        const typeMatch = sqlType.toLowerCase();

        const stringTypes = [
            "text", "varchar", "char", "character", "character varying", "name",
            "uuid", "citext", "xml", "ltree", "lquery", "ltxtquery", "tsvector",
            "tsquery", "bytea", "bit", "bit varying", "varbit", "inet", "cidr",
            "macaddr", "macaddr8", "oid", "regproc", "regprocedure", "regoper",
            "regoperator", "regclass", "regtype", "regconfig", "regdictionary"
        ];

        const numberTypes = [
            "integer", "int", "int4", "int8", "smallint", "int2", "bigint",
            "decimal", "numeric", "real", "float4", "double precision", "float8",
            "serial", "bigserial", "smallserial", "serial2", "serial4", "serial8", "money"
        ];

        const booleanTypes = ["boolean", "bool"];

        const dateTypes = [
            "timestamp", "timestamptz", "timestamp with time zone",
            "timestamp without time zone", "date", "time", "time with time zone",
            "time without time zone", "timetz", "interval"
        ];

        const objectTypes = [
            "json", "jsonb", "point", "line", "lseg", "box", "path", "polygon",
            "circle", "int4range", "int8range", "numrange", "tsrange", "tstzrange", "daterange"
        ];

        if (stringTypes.includes(typeMatch)) return "string";
        if (numberTypes.includes(typeMatch)) return "number";
        if (booleanTypes.includes(typeMatch)) return "boolean";
        if (dateTypes.includes(typeMatch)) return "Date";
        if (objectTypes.includes(typeMatch)) return "object";
        if (typeMatch.endsWith("[]")) return "array";

        return "any";
    }
}
