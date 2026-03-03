//! Generate table order and circular FK columns from a live database (same logic as order-tables binary).

use std::collections::{HashMap, HashSet, VecDeque};

/// Tables to exclude from migration order (must match store SYSTEM_TABLES that are not commented out).
fn system_tables_set() -> HashSet<&'static str> {
    [
        "crdt_messages",
        "crdt_merkles",
        "sync_endpoints",
        "queues",
        "queue_items",
        "transactions",
        "entities",
        "fields",
        "entity_fields",
        "permissions",
        "encryption_keys",
        "data_permissions",
        "role_permissions",
        "organization_domains",
        "system_config_fields",
        "record_permissions",
        "role_permission",
        "table_indexes",
        "stream_queue",
        "stream_queue_items",
        "test",
    ]
    .into_iter()
    .collect()
}

fn ensure_sslmode_disable(url: &str) -> String {
    if url.contains("sslmode=") {
        url.to_string()
    } else {
        let sep = if url.contains('?') { "&" } else { "?" };
        format!("{}{}sslmode=disable", url, sep)
    }
}

async fn fetch_tables_in_schema(
    client: &tokio_postgres::Client,
    schema: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = client
        .query(
            r#"
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = $1
              AND table_type = 'BASE TABLE'
            ORDER BY table_name
            "#,
            &[&schema],
        )
        .await?;
    Ok(rows
        .iter()
        .map(|r| r.get::<_, String>("table_name"))
        .collect())
}

/// Returns (from_table, to_table, from_column): from_table.from_column FK → to_table.
async fn fetch_fk_relations(
    client: &tokio_postgres::Client,
    schema: &str,
) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = client
        .query(
            r#"
            SELECT
                src.relname   AS from_table,
                tgt.relname   AS to_table,
                att.attname   AS from_column
            FROM pg_constraint con
            JOIN pg_class src ON src.oid = con.conrelid
            JOIN pg_class tgt ON tgt.oid = con.confrelid
            JOIN pg_namespace nsp ON nsp.oid = src.relnamespace
            JOIN pg_attribute att
                ON att.attrelid = con.conrelid
                AND att.attnum = ANY(con.conkey)
                AND att.attnum > 0
                AND NOT att.attisdropped
            WHERE con.contype = 'f'
              AND nsp.nspname = $1
            "#,
            &[&schema],
        )
        .await?;

    let mut triples = Vec::new();
    for row in rows {
        let from_table: String = row.get("from_table");
        let to_table: String = row.get("to_table");
        let from_column: String = row.get("from_column");
        triples.push((from_table, to_table, from_column));
    }
    Ok(triples)
}

/// Fetch unique (non-PK) indexes for the given tables from the DB.
/// Returns table -> [(index_name, full CREATE INDEX DDL)].
async fn fetch_unique_indexes(
    client: &tokio_postgres::Client,
    schema: &str,
    tables: &[String],
) -> Result<HashMap<String, Vec<(String, String)>>, Box<dyn std::error::Error + Send + Sync>> {
    let table_set: HashSet<&str> = tables.iter().map(String::as_str).collect();
    let table_set_lower: std::collections::HashSet<String> =
        tables.iter().map(|s| s.to_lowercase()).collect();

    let rows = {
        let r = client
            .query(
                r#"
                SELECT
                    c.relname AS table_name,
                    i.relname AS index_name,
                    pg_get_indexdef(ix.indexrelid) AS index_def
                FROM pg_index ix
                JOIN pg_class i ON i.oid = ix.indexrelid
                JOIN pg_class c ON c.oid = ix.indrelid
                JOIN pg_namespace n ON n.oid = c.relnamespace
                WHERE n.nspname = $1
                  AND c.relkind = 'r'
                  AND ix.indisunique
                  AND NOT ix.indisprimary
                "#,
                &[&schema],
            )
            .await?;
        if r.is_empty() {
            // Fallback: pg_indexes view
            client
                .query(
                    r#"
                    SELECT tablename AS table_name, indexname AS index_name, indexdef AS index_def
                    FROM pg_indexes
                    WHERE schemaname = $1
                      AND indexdef LIKE '%UNIQUE%'
                      AND indexname NOT LIKE '%_pkey'
                    "#,
                    &[&schema],
                )
                .await?
        } else {
            r
        }
    };

    let total_rows = rows.len();
    let mut out: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for row in rows {
        let table_name: String = row.get("table_name");
        // Only include tables in the migration list
        if !table_set.contains(table_name.as_str())
            && !table_set_lower.contains(&table_name.to_lowercase())
        {
            continue;
        }
        let index_name: String = row.get("index_name");
        let index_def: String = row.get("index_def");
        // Use the canonical name from the migration list
        let key = tables
            .iter()
            .find(|t| t.as_str() == table_name || t.to_lowercase() == table_name.to_lowercase())
            .cloned()
            .unwrap_or(table_name);
        out.entry(key).or_default().push((index_name, index_def));
    }
    if total_rows > 0 && out.is_empty() {
        eprintln!(
            "order-tables: {} unique index rows from DB but none matched the {} migration tables (schema {:?}).",
            total_rows, tables.len(), schema
        );
    } else {
        eprintln!(
            "order-tables: found {} unique index(es) across {} table(s) from {} total rows (schema {:?})",
            out.values().map(|v| v.len()).sum::<usize>(),
            out.len(),
            total_rows,
            schema
        );
    }
    Ok(out)
}

fn topological_sort(
    tables: &[String],
    fk_edges: &[(String, String, String)],
) -> Result<(Vec<String>, HashMap<String, Vec<String>>), Box<dyn std::error::Error + Send + Sync>> {
    let table_set: HashSet<&str> = tables.iter().map(String::as_str).collect();

    let edges_set: HashSet<(&str, &str)> = fk_edges
        .iter()
        .filter_map(|(from, to, _)| {
            if table_set.contains(from.as_str()) && table_set.contains(to.as_str()) {
                Some((to.as_str(), from.as_str()))
            } else {
                None
            }
        })
        .collect();
    let edges: Vec<(&str, &str)> = edges_set.into_iter().collect();

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(u, v) in &edges {
        adj.entry(u).or_default().push(v);
    }

    let mut index: usize = 0;
    let mut stack: Vec<&str> = Vec::new();
    let mut indices: HashMap<&str, usize> = HashMap::new();
    let mut lowlinks: HashMap<&str, usize> = HashMap::new();
    let mut on_stack: HashSet<&str> = HashSet::new();
    let mut sccs: Vec<Vec<&str>> = Vec::new();

    fn strong_connect<'a>(
        u: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        index: &mut usize,
        stack: &mut Vec<&'a str>,
        indices: &mut HashMap<&'a str, usize>,
        lowlinks: &mut HashMap<&'a str, usize>,
        on_stack: &mut HashSet<&'a str>,
        sccs: &mut Vec<Vec<&'a str>>,
    ) {
        *index += 1;
        let i = *index;
        indices.insert(u, i);
        lowlinks.insert(u, i);
        stack.push(u);
        on_stack.insert(u);

        for &v in adj.get(u).into_iter().flatten() {
            if !indices.contains_key(v) {
                strong_connect(v, adj, index, stack, indices, lowlinks, on_stack, sccs);
                let lu = *lowlinks.get(u).unwrap();
                let lv = *lowlinks.get(v).unwrap();
                lowlinks.insert(u, lu.min(lv));
            } else if on_stack.contains(v) {
                let lu = *lowlinks.get(u).unwrap();
                let iv = *indices.get(v).unwrap();
                lowlinks.insert(u, lu.min(iv));
            }
        }

        if *lowlinks.get(u).unwrap() == *indices.get(u).unwrap() {
            let mut scc = Vec::new();
            loop {
                let v = stack.pop().unwrap();
                on_stack.remove(v);
                scc.push(v);
                if v == u {
                    break;
                }
            }
            sccs.push(scc);
        }
    }

    for t in tables.iter().map(String::as_str) {
        if !indices.contains_key(t) {
            strong_connect(
                t,
                &adj,
                &mut index,
                &mut stack,
                &mut indices,
                &mut lowlinks,
                &mut on_stack,
                &mut sccs,
            );
        }
    }

    let mut node_to_scc: HashMap<&str, usize> = HashMap::new();
    for (id, scc) in sccs.iter().enumerate() {
        for &node in scc {
            node_to_scc.insert(node, id);
        }
    }

    let mut circular_fk_cols: HashMap<String, HashSet<String>> = HashMap::new();
    for (from, to, col) in fk_edges {
        if !table_set.contains(from.as_str()) {
            continue;
        }
        let &scc_from = match node_to_scc.get(from.as_str()) {
            Some(s) => s,
            None => continue,
        };
        let &scc_to = match node_to_scc.get(to.as_str()) {
            Some(s) => s,
            None => continue,
        };
        if scc_from == scc_to {
            circular_fk_cols
                .entry(from.clone())
                .or_default()
                .insert(col.clone());
        }
    }
    let circular_fk_cols: HashMap<String, Vec<String>> = circular_fk_cols
        .into_iter()
        .map(|(t, cols)| {
            let mut v: Vec<String> = cols.into_iter().collect();
            v.sort();
            (t, v)
        })
        .collect();

    let mut cond_adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for (id, _) in sccs.iter().enumerate() {
        cond_adj.entry(id).or_default();
    }
    for &(u, v) in &edges {
        let id_u = node_to_scc[u];
        let id_v = node_to_scc[v];
        if id_u != id_v {
            cond_adj.get_mut(&id_u).unwrap().push(id_v);
        }
    }

    let mut cond_in_degree: HashMap<usize, usize> = (0..sccs.len()).map(|i| (i, 0)).collect();
    for neighbors in cond_adj.values() {
        for &v in neighbors {
            *cond_in_degree.get_mut(&v).unwrap() += 1;
        }
    }

    let tables_with_fk: HashSet<&str> = fk_edges.iter().map(|(from, _, _)| from.as_str()).collect();

    let mut roots: Vec<usize> = cond_in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&i, _)| i)
        .collect();
    roots.sort_by_key(|&id| {
        let scc_tables = &sccs[id];
        let has_outgoing = cond_adj.get(&id).map(|v| !v.is_empty()).unwrap_or(false);
        let any_has_fk = scc_tables.iter().any(|&t| tables_with_fk.contains(t));
        match (any_has_fk, has_outgoing) {
            (false, false) => 0,
            (_, true) => 1,
            _ => 2,
        }
    });
    let mut queue: VecDeque<usize> = VecDeque::from(roots);

    let mut scc_order: Vec<usize> = Vec::with_capacity(sccs.len());
    while let Some(id) = queue.pop_front() {
        scc_order.push(id);
        for &v in &cond_adj[&id] {
            let d = cond_in_degree.get_mut(&v).unwrap();
            *d -= 1;
            if *d == 0 {
                queue.push_back(v);
            }
        }
    }

    let mut order = Vec::with_capacity(tables.len());
    for id in scc_order {
        for &t in &sccs[id] {
            order.push(t.to_string());
        }
    }
    Ok((order, circular_fk_cols))
}

fn escape_rust_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}

/// Generate Rust source for MIGRATE_UNIQUE_INDEXES (table -> [(index_name, create_ddl)]).
fn format_unique_indexes_rust(unique_indexes: &HashMap<String, Vec<(String, String)>>) -> String {
    let mut out = String::from(
        "\n// Unique indexes to drop on destination before migrating each table and recreate after.\n\
         pub const MIGRATE_UNIQUE_INDEXES: &[(&str, &[(&str, &str)])] = &[\n",
    );
    let mut tables: Vec<&String> = unique_indexes.keys().collect();
    tables.sort();
    for table in tables {
        let indexes = &unique_indexes[table];
        let mut entries = Vec::with_capacity(indexes.len());
        for (name, ddl) in indexes {
            entries.push(format!(
                "    (\"{}\", \"{}\")",
                escape_rust_str(name),
                escape_rust_str(ddl)
            ));
        }
        out.push_str(&format!("    (\"{}\", &[\n", escape_rust_str(table)));
        out.push_str(&entries.join(",\n"));
        out.push_str("\n    ]),\n");
    }
    out.push_str("];\n");
    out
}

/// Generate Rust source content for migrate_tables_order.rs.
pub fn format_rust_output(
    tables: &[String],
    circular_fk_cols: &HashMap<String, Vec<String>>,
    unique_indexes: &HashMap<String, Vec<(String, String)>>,
) -> String {
    let mut out = String::from(
        "// Generated by migrate-from-db order-tables - do not edit by hand\n\
         // Tables in dependency order (referenced tables first). Use for MIGRATE_TABLES.\n\n\
         pub const MIGRATE_TABLES_ORDER: &[&str] = &[\n",
    );
    for t in tables {
        out.push_str(&format!("    \"{}\",\n", escape_rust_str(t)));
    }
    out.push_str("];\n\n");

    out.push_str(
        "// Columns that must be set to NULL on first insert (circular FK cycles).\n\
         // After all tables are inserted, a second pass will restore these values.\n\
         pub const MIGRATE_CIRCULAR_FK_COLS: &[(&str, &[&str])] = &[\n",
    );
    let mut sorted_tables: Vec<&String> = circular_fk_cols.keys().collect();
    sorted_tables.sort();
    for table in sorted_tables {
        let cols = &circular_fk_cols[table];
        let cols_str = cols
            .iter()
            .map(|c| format!("\"{}\"", escape_rust_str(c)))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "    (\"{}\", &[{}]),\n",
            escape_rust_str(table),
            cols_str
        ));
    }
    out.push_str("];\n");
    out.push_str(&format_unique_indexes_rust(unique_indexes));
    out
}

/// Connect to the database, fetch tables and FKs, and return (sorted_tables, circular_fk_cols, unique_indexes).
/// Uses read-only connection. Excludes system tables.
/// If dest_database_url is Some, unique indexes are read from the destination DB (where we drop/recreate them); otherwise from source.
pub async fn generate_table_order(
    database_url: &str,
    schema: &str,
    dest_database_url: Option<&str>,
) -> Result<
    (
        Vec<String>,
        HashMap<String, Vec<String>>,
        HashMap<String, Vec<(String, String)>>,
    ),
    Box<dyn std::error::Error + Send + Sync>,
> {
    let connect_url = ensure_sslmode_disable(database_url);
    let (client, connection) = tokio_postgres::connect(&connect_url, tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        let _ = connection.await;
    });
    client
        .batch_execute("SET default_transaction_read_only = on")
        .await?;

    let all_tables = fetch_tables_in_schema(&client, schema).await?;
    let system = system_tables_set();
    let tables: Vec<String> = all_tables
        .into_iter()
        .filter(|t| !system.contains(t.as_str()))
        .collect();

    let fk_edges = fetch_fk_relations(&client, schema).await?;
    let (sorted, circular_fk_cols) = topological_sort(&tables, &fk_edges)?;

    let unique_indexes = if let Some(dest_url) = dest_database_url {
        let dest_connect = ensure_sslmode_disable(dest_url);
        let (dest_client, dest_conn) =
            tokio_postgres::connect(&dest_connect, tokio_postgres::NoTls).await?;
        tokio::spawn(async move {
            let _ = dest_conn.await;
        });
        dest_client
            .batch_execute("SET default_transaction_read_only = on")
            .await?;
        fetch_unique_indexes(&dest_client, schema, &sorted).await?
    } else {
        fetch_unique_indexes(&client, schema, &sorted).await?
    };

    Ok((sorted, circular_fk_cols, unique_indexes))
}
