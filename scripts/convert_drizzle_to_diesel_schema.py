# Use this only for converting TS Drizzle schema to Rust DIESEL schema
import re
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple

root = Path("/Users/chaosumaru/Documents/projects/Connectivo Workspace/datastore-connectivo")
ts_dir = root / "datastore-postgres-experimental-qa-src-schema-application/src/schema/application"
rs_dir = root / "bin/store/src/database/schema/application_tables"

SYSTEM_FIELDS_IGNORED_FOR_CUSTOM_INDEXES: Set[str] = {
    "tombstone",
    "status",
    "previous_status",
    "version",
    "created_date",
    "created_time",
    "updated_date",
    "updated_time",
    "organization_id",
    "created_by",
    "updated_by",
    "deleted_by",
    "requested_by",
    "timestamp",
    "tags",
    "categories",
    "code",
    "id",
    "sensitivity_level",
    "sync_status",
    "is_batch",
    "image_url",
}


def snake_to_pascal(s: str) -> str:
    return "".join(p.capitalize() for p in s.split("_"))


def map_base_type(expr: str) -> str:
    c = expr.lower()
    compact = re.sub(r"\s+", "", c)
    if "doubleprecision(" in c or "double_precision(" in c:
        return "doubleprecision()"
    if "smallint(" in c:
        return "smallint()"
    if "bigint(" in c:
        return "bigint()"
    if "serial(" in c or "integer(" in c:
        return "integer()"
    if "numeric(" in c or "decimal(" in c:
        return "integer()"
    if "boolean(" in c:
        return "boolean()"
    if "jsonb(" in c:
        return "jsonb()"
    if "json(" in c:
        return "json()"
    if "uuid(" in c:
        return "uuid()"
    if "date(" in c:
        return "date()"
    if "time(" in c and "timestamp" not in c and "timestamptz" not in c:
        return "time()"
    if "timestamptz(" in c:
        return "timestamptz()"
    if "timestamp(" in c and "withtimezone:true" in compact:
        return "timestamptz()"
    if "timestamp(" in c:
        return "timestamp()"
    if "varchar(" in c:
        m = re.search(r"varchar\((\d+)\)", c)
        return f"varchar(Some({m.group(1)}))" if m else "varchar(None)"
    if "char(" in c:
        m = re.search(r"char\((\d+)\)", c)
        return f"char(Some({m.group(1)}))" if m else "char(None)"
    return "text()"


def extract_block(text: str, start_idx: int, open_char="{", close_char="}") -> str:
    depth = 0
    i = start_idx
    in_str = None
    while i < len(text):
        ch = text[i]
        if in_str is not None:
            if ch == "\\":
                i += 2
                continue
            if ch == in_str:
                in_str = None
            i += 1
            continue
        if ch in ('"', "'", "`"):
            in_str = ch
            i += 1
            continue
        if ch == open_char:
            depth += 1
        elif ch == close_char:
            depth -= 1
            if depth == 0:
                return text[start_idx + 1 : i]
        i += 1
    return ""


def parse_import_alias_map(ts: str) -> Dict[str, str]:
    alias_to_table: Dict[str, str] = {}
    for m in re.finditer(r"import\s*\{([^}]+)\}\s*from\s*'([^']+)'", ts):
        names = m.group(1)
        from_path = m.group(2)
        from_table = Path(from_path).name
        for part in [p.strip() for p in names.split(",") if p.strip()]:
            as_m = re.match(r"table\s+as\s+([A-Za-z_][A-Za-z0-9_]*)", part)
            if as_m:
                alias_to_table[as_m.group(1)] = from_table
            else:
                plain = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", part)
                if plain:
                    alias_to_table[plain.group(1)] = plain.group(1)
    return alias_to_table


def parse_index_exports(index_ts: str) -> List[Tuple[str, str]]:
    exports: List[Tuple[str, str]] = []
    for line in index_ts.splitlines():
        s = line.strip()
        if not s or s.startswith("//"):
            continue
        m = re.match(
            r"export\s*\{\s*table\s+as\s+([A-Za-z_][A-Za-z0-9_]*)\s*\}\s*from\s*['\"]\./([^'\"]+)['\"]\s*;",
            s,
        )
        if m:
            exports.append((m.group(1), m.group(2)))
    return exports


def parse_enum_values(ts: str) -> Dict[str, str]:
    enum_values: Dict[str, str] = {}
    for m in re.finditer(r"\bexport\s+enum\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", ts):
        enum_name = m.group(1)
        block = extract_block(ts, m.end() - 1)
        for line in block.splitlines():
            s = line.strip().rstrip(",")
            if not s or s.startswith("//"):
                continue
            mm = re.match(r"([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(['\"])(.*?)\2$", s)
            if not mm:
                continue
            member = mm.group(1)
            value = mm.group(3)
            enum_values[f"{enum_name}.{member}"] = value
    return enum_values


def parse_object_entries(block: str) -> List[Tuple[str, str]]:
    def strip_ts_line_comment(line: str) -> str:
        in_str: Optional[str] = None
        i = 0
        kept: List[str] = []
        while i < len(line):
            ch = line[i]
            if in_str is not None:
                kept.append(ch)
                if ch == "\\":
                    if i + 1 < len(line):
                        kept.append(line[i + 1])
                        i += 2
                        continue
                elif ch == in_str:
                    in_str = None
                i += 1
                continue
            if ch in ("'", '"', "`"):
                in_str = ch
                kept.append(ch)
                i += 1
                continue
            if ch == "/" and i + 1 < len(line) and line[i + 1] == "/":
                break
            kept.append(ch)
            i += 1
        return "".join(kept)

    def has_balanced_delimiters(text: str) -> bool:
        pairs = {"(": ")", "[": "]", "{": "}"}
        closing = set(pairs.values())
        stack: List[str] = []
        in_str: Optional[str] = None
        i = 0
        while i < len(text):
            ch = text[i]
            if in_str is not None:
                if ch == "\\":
                    i += 2
                    continue
                if ch == in_str:
                    in_str = None
                i += 1
                continue
            if ch in ("'", '"', "`"):
                in_str = ch
                i += 1
                continue
            if ch in pairs:
                stack.append(pairs[ch])
            elif ch in closing:
                if not stack or ch != stack.pop():
                    return False
            i += 1
        return not stack and in_str is None

    lines = block.splitlines()
    out: List[Tuple[str, str]] = []
    i = 0
    while i < len(lines):
        s = lines[i].strip()
        if not s or s.startswith("//") or s.startswith("..."):
            i += 1
            continue
        m = re.match(r"([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+)", s)
        if not m:
            i += 1
            continue
        key = m.group(1)
        expr = m.group(2)
        while (
            not (
                strip_ts_line_comment(expr).rstrip().endswith(",")
                and has_balanced_delimiters(strip_ts_line_comment(expr).rstrip().rstrip(","))
            )
            and i + 1 < len(lines)
        ):
            i += 1
            expr += " " + lines[i].strip()
        cleaned = strip_ts_line_comment(expr).rstrip()
        out.append((key, cleaned.rstrip(",").strip()))
        i += 1
    return out


def extract_reference(expr: str) -> Optional[Tuple[str, str]]:
    m = re.search(r"\.references\((.+)\)", expr)
    if not m or "=>" not in m.group(1):
        return None
    rhs = m.group(1).split("=>", 1)[1].strip()
    rhs = rhs.rstrip(") ").strip().rstrip(",").strip()
    cast_m = re.search(
        r"\(([A-Za-z_][A-Za-z0-9_]*)\s+as[\s\S]*?\)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)",
        rhs,
    )
    if cast_m:
        return cast_m.group(1), cast_m.group(2)
    dot_m = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)", rhs)
    if dot_m:
        return dot_m.group(1), dot_m.group(2)
    return None


def parse_default(expr: str, enum_values: Dict[str, str]) -> Optional[str]:
    m = re.search(r"\.default\(([^)]+)\)", expr)
    if not m:
        return None
    v = m.group(1).strip()
    if (v.startswith("'") and v.endswith("'")) or (v.startswith('"') and v.endswith('"')):
        v = v[1:-1]
    v = v.rstrip(",").strip()
    enum_ref = re.match(r"^([A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*)$", v)
    if enum_ref:
        resolved = enum_values.get(enum_ref.group(1))
        if resolved is not None:
            v = resolved
    # Convert TS array literal defaults like ['Full Site'] to Postgres array literal style.
    if v.startswith("[") and v.endswith("]"):
        inner = v[1:-1].strip()
        if not inner:
            return "'{}'"
        items = [item.strip() for item in inner.split(",")]
        cleaned: List[str] = []
        for item in items:
            if (item.startswith("'") and item.endswith("'")) or (
                item.startswith('"') and item.endswith('"')
            ):
                cleaned.append(item[1:-1])
            else:
                cleaned.append(item)
        return "'{" + ",".join(cleaned) + "}'"
    return v


def parse_fields(ts: str, alias_map: Dict[str, str]) -> Tuple[List[Tuple[str, str, Optional[str]]], List[Tuple[str, str, str]]]:
    enum_values = parse_enum_values(ts)
    pairs: List[Tuple[str, str]] = []
    for m in re.finditer(r"\bconst\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\{", ts):
        var = m.group(1)
        var_lower = var.lower()
        if var_lower != "fields" and not var_lower.endswith("fields"):
            continue
        pairs.extend(parse_object_entries(extract_block(ts, m.end() - 1)))

    for m in re.finditer(r"pgTable\s*\(", ts):
        comma = ts.find(",", m.end())
        if comma == -1:
            continue
        brace = ts.find("{", comma)
        if brace == -1:
            continue
        pairs.extend(parse_object_entries(extract_block(ts, brace)))

    fields: List[Tuple[str, str, Optional[str]]] = []
    seen: Set[str] = set()
    fk_map: Dict[str, Tuple[str, str]] = {}

    for key, expr in pairs:
        if key in ("system_fields", "fields"):
            continue
        dbm = re.search(r"\w+\(\s*'([^']+)'", expr)
        col = dbm.group(1) if dbm else key
        if col in SYSTEM_FIELDS_IGNORED_FOR_CUSTOM_INDEXES:
            continue

        ref = extract_reference(expr)
        if ref:
            alias, target_col = ref
            fk_map[col] = (alias_map.get(alias, alias), target_col)

        if col in seen:
            continue
        seen.add(col)

        base = map_base_type(expr)
        typ = f"nullable({base})"
        if ".array()" in expr.replace(" ", ""):
            typ = f"nullable(array({base}))"
        default = parse_default(expr, enum_values)
        if default is not None and base == "jsonb()":
            compact_expr = expr.replace(" ", "")
            if ".default([])" in compact_expr:
                default = "'[]'::jsonb"
            elif ".default({})" in compact_expr:
                default = "'{}'::jsonb"
            else:
                d = default.strip()
                if "::jsonb" not in d.lower():
                    if d in ("[]", "'[]'"):
                        default = "'[]'::jsonb"
                    elif d in ("{}", "'{}'"):
                        default = "'{}'::jsonb"
                    else:
                        default = f"'{d}'::jsonb"
        fields.append((col, typ, default))

    fk_list = [(col, tbl, tcol) for col, (tbl, tcol) in fk_map.items()]
    return fields, fk_list


def parse_custom_indexes(ts: str, field_names: Set[str]) -> List[str]:
    def strip_ts_line_comments(text: str) -> str:
        output_lines: List[str] = []
        for line in text.splitlines():
            in_str: Optional[str] = None
            i = 0
            kept: List[str] = []
            while i < len(line):
                ch = line[i]
                if in_str is not None:
                    kept.append(ch)
                    if ch == "\\":
                        if i + 1 < len(line):
                            kept.append(line[i + 1])
                            i += 2
                            continue
                    elif ch == in_str:
                        in_str = None
                    i += 1
                    continue
                if ch in ("'", '"', "`"):
                    in_str = ch
                    kept.append(ch)
                    i += 1
                    continue
                if ch == "/" and i + 1 < len(line) and line[i + 1] == "/":
                    break
                kept.append(ch)
                i += 1
            output_lines.append("".join(kept))
        return "\n".join(output_lines)

    if "index(" not in ts:
        return []
    searchable: List[str] = []
    m = re.search(r"const\s+searchable_fields\s*=\s*\[([\s\S]*?)\]", ts)
    if m:
        raw_block = m.group(1)
        clean_block = strip_ts_line_comments(raw_block)
        searchable = re.findall(r"'([^']+)'", clean_block)
    elif "Object.keys({ ...system_fields, ...fields }).reduce" in ts:
        searchable = list(field_names)
    if not searchable:
        return []
    excludes = re.findall(r"index_name\.includes\('([^']+)'\)", ts)
    out: List[str] = []
    seen: Set[str] = set()
    for field in searchable:
        if field not in field_names:
            continue
        if field in SYSTEM_FIELDS_IGNORED_FOR_CUSTOM_INDEXES:
            continue
        if "_time" in field:
            continue
        if any(ex in field for ex in excludes):
            continue
        if field in seen:
            continue
        seen.add(field)
        out.append(field)
    return out


def parse_explicit_indexes(ts: str, field_names: Set[str]) -> List[Dict[str, object]]:
    explicit: List[Dict[str, object]] = []
    pattern = re.compile(
        r"(uniqueIndex|index)\s*\(\s*(?:`([^`]+)`|'([^']+)'|\"([^\"]+)\")\s*\)\s*\.on\(([\s\S]*?)\)",
        re.MULTILINE,
    )
    for m in pattern.finditer(ts):
        kind = m.group(1)
        idx_name = m.group(2) or m.group(3) or m.group(4) or "idx"
        args = m.group(5)
        cols = re.findall(r"table\.([A-Za-z_][A-Za-z0-9_]*)", args)
        cols = [c for c in cols if c in field_names]
        if any(c in SYSTEM_FIELDS_IGNORED_FOR_CUSTOM_INDEXES for c in cols):
            continue
        if not cols:
            continue
        explicit.append(
            {
                "name": idx_name,
                "columns": cols,
                "unique": kind == "uniqueIndex",
            }
        )
    return explicit


def main() -> None:
    rs_dir.mkdir(parents=True, exist_ok=True)
    updated = 0
    targets: List[Tuple[str, Path]] = []
    index_ts_file = ts_dir / "index.ts"
    if index_ts_file.exists():
        for table, source_stem in parse_index_exports(index_ts_file.read_text()):
            ts_file = ts_dir / f"{source_stem}.ts"
            if ts_file.exists():
                targets.append((table, ts_file))
    else:
        for ts_file in sorted(ts_dir.glob("*.ts")):
            if ts_file.name == "index.ts":
                continue
            targets.append((ts_file.stem, ts_file))

    for table, ts_file in targets:
        rs_file = rs_dir / f"{table}.rs"
        ts = ts_file.read_text()
        alias_map = parse_import_alias_map(ts)
        fields, fk_list = parse_fields(ts, alias_map)
        field_names = {n for n, _, _ in fields}
        indexes = parse_custom_indexes(ts, field_names)
        explicit_indexes = parse_explicit_indexes(ts, field_names)

        lines = [
            "use crate::define_table_schema;",
            "use crate::generated::schema::generator::diesel_schema_definition::{",
            "    types::*, DieselTableDefinition,",
            "};",
            "use crate::{system_fields, system_foreign_keys, system_indexes};",
            "",
            "/**",
            " * Auto-converted from TypeScript schema source.",
            " */",
            f"pub struct {snake_to_pascal(table)}Table;",
            "",
            "define_table_schema! {",
            "    hypertable: false,",
            "    fields: {",
            "        // System fields - common across all tables ( REQUIRED )",
            "        system_fields!(),",
            "",
        ]

        for name, typ, default in fields:
            if default is not None:
                lines.append(f'        {name}: {typ}, default "{default}",')
            else:
                lines.append(f"        {name}: {typ},")

        lines.extend(
            [
                "    },",
                "    indexes: {",
                "        // System field indexes ( REQUIRED )",
                f'        system_indexes!("{table}"),',
            ]
        )

        if indexes or explicit_indexes:
            lines.append("        // Custom table-specific indexes")
            for field in indexes:
                lines.extend(
                    [
                        f"        idx_{table}_{field}: {{",
                        f'            columns: ["{field}"],',
                        "            unique: false,",
                        '            type: "btree"',
                        "        },",
                    ]
                )
            for ex in explicit_indexes:
                cols = ex["columns"]
                unique = bool(ex["unique"])
                cols_key = "_".join([re.sub(r"[^A-Za-z0-9_]", "_", c).strip("_").lower() for c in cols])
                key = f"idx_{table}_{cols_key}" if cols_key else f"idx_{table}"
                cols_literal = ", ".join([f'"{c}"' for c in cols])
                lines.extend(
                    [
                        f"        {key}: {{",
                        f"            columns: [{cols_literal}],",
                        f"            unique: {'true' if unique else 'false'},",
                        '            type: "btree"',
                        "        },",
                    ]
                )

        lines.extend(
            [
                "    },",
                "    foreign_keys: {",
                "        // System field foreign keys ( REQUIRED )",
                f'        system_foreign_keys!("{table}"),',
            ]
        )

        for col, foreign_table, foreign_col in fk_list:
            lines.append(
                f'        fk_{table}_{col}: {{ columns: ["{col}"], foreign_table: "{foreign_table}", foreign_columns: ["{foreign_col}"], on_delete: "no action", on_update: "no action" }},'
            )

        lines.extend(["    }", "}", ""])
        rs_file.write_text("\n".join(lines))
        updated += 1

    print(f"Updated {updated} files")


if __name__ == "__main__":
    main()
