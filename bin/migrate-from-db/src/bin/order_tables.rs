//! Connects to the database, reads all tables and foreign-key relations from
//! information_schema, topologically sorts tables so dependencies (referenced tables)
//! come first, and writes the result as a Rust array to a file.
//!
//! Env:
//! - MIGRATE_FROM_DATABASE_URL (or MIGRATE_ORDER_DATABASE_URL) - Postgres URL
//! - MIGRATE_ORDER_SCHEMA       - Schema to consider (default: public)
//! - MIGRATE_ORDER_OUTPUT       - Output file path (default: migrate_tables_order.rs)
//!
//! System tables (see bin/store/src/database/schema/system_tables.rs) are excluded from output.

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
        "counters",
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
use std::env;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let database_url = env::var("MIGRATE_ORDER_DATABASE_URL")
        .or_else(|_| env::var("MIGRATE_FROM_DATABASE_URL"))
        .map_err(|_| "MIGRATE_FROM_DATABASE_URL or MIGRATE_ORDER_DATABASE_URL required")?;
    let schema = env::var("MIGRATE_ORDER_SCHEMA").unwrap_or_else(|_| "public".to_string());
    let output_path =
        env::var("MIGRATE_ORDER_OUTPUT").unwrap_or_else(|_| "migrate_tables_order.rs".to_string());

    // Force no TLS so local Postgres without SSL works (avoids "server does not support TLS")
    let connect_url = ensure_sslmode_disable(&database_url);

    let (client, connection) = tokio_postgres::connect(&connect_url, tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        let _ = connection.await;
    });
    client
        .batch_execute("SET default_transaction_read_only = on")
        .await?;

    let all_tables = fetch_tables_in_schema(&client, &schema).await?;
    let n_all = all_tables.len();
    let system = system_tables_set();
    let tables: Vec<String> = all_tables
        .into_iter()
        .filter(|t| !system.contains(t.as_str()))
        .collect();
    println!(
        "Excluded {} system tables; {} tables to migrate",
        n_all.saturating_sub(tables.len()),
        tables.len()
    );

    let fk_edges = fetch_fk_relations(&client, &schema).await?;

    // Build dependency graph: for FK (from_table -> to_table), we must insert to_table before from_table.
    // So we have edge to_table -> from_table (to_table is dependency of from_table).
    // Topological order: dependencies first, so we want an order where for every edge (u -> v), u comes before v.
    let (sorted, circular_fk_cols) = topological_sort(&tables, &fk_edges)?;

    let content = format_rust_output(&sorted, &circular_fk_cols);
    let mut f = File::create(&output_path)?;
    f.write_all(content.as_bytes())?;
    println!("Wrote {} tables to {}", sorted.len(), output_path);

    Ok(())
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
/// So to_table must be inserted before from_table.
async fn fetch_fk_relations(
    client: &tokio_postgres::Client,
    schema: &str,
) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = client
        .query(
            r#"
            SELECT
                tc.table_name   AS from_table,
                ccu.table_name AS to_table,
                kcu.column_name AS from_column
            FROM information_schema.table_constraints tc
            JOIN information_schema.key_column_usage kcu
                ON tc.constraint_name = kcu.constraint_name
                AND tc.table_schema = kcu.table_schema
            JOIN information_schema.constraint_column_usage ccu
                ON ccu.constraint_name = tc.constraint_name
                AND ccu.table_schema = tc.table_schema
            WHERE tc.constraint_type = 'FOREIGN KEY'
              AND tc.table_schema = $1
              AND ccu.table_schema = $1
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

/// Order tables so that for every FK (from -> to), `to` appears before `from` when possible.
/// When FKs form a cycle (e.g. A->B->A or self-reference), we still produce an order by
/// treating each strongly connected component (SCC) as a block and ordering blocks topologically.
/// Also returns circular_fk_cols: for each table, columns that are part of a cycle (same SCC or self-ref)
/// and should be set to NULL on first insert, then updated in a second pass.
fn topological_sort(
    tables: &[String],
    fk_edges: &[(String, String, String)],
) -> Result<(Vec<String>, HashMap<String, Vec<String>>), Box<dyn std::error::Error + Send + Sync>> {
    let table_set: HashSet<&str> = tables.iter().map(String::as_str).collect();

    // Edges: (to, from) meaning to must come before from. Only in-schema, deduped.
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

    // Adjacency: u -> [v] means u must be before v
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(u, v) in &edges {
        adj.entry(u).or_default().push(v);
    }

    // Tarjan's SCC; we need reverse graph for "dependency order" (roots first)
    // Our graph: edge (u,v) = u before v. We want order with dependencies first, so we output in reverse DFS post-order of the graph. Tarjan gives SCCs; we sort SCCs by dependency (SCC A before SCC B if there's an edge from B to A in our graph = B depends on A, so A first). So we need condensation graph and then topological order of condensation where "dependency" SCC comes first.
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

    // Assign each node its SCC id (index in sccs)
    let mut node_to_scc: HashMap<&str, usize> = HashMap::new();
    for (id, scc) in sccs.iter().enumerate() {
        for &node in scc {
            node_to_scc.insert(node, id);
        }
    }

    // Detect circular FK columns: FKs where both tables are in the same SCC (cycle) or self-reference.
    // These columns must be set to NULL on first insert, then updated in a second pass.
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

    // Condensation graph: SCC id -> list of SCC ids that depend on it (edge from this SCC to dependent)
    // We have graph edge u -> v (u before v). So from u's SCC to v's SCC there's an edge. For topological order of SCCs we want "if edge A -> B then A before B". So we need in-degrees for condensation and sort.
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

    // In-degree of condensation nodes (how many SCCs must come before this one)
    let mut cond_in_degree: HashMap<usize, usize> = (0..sccs.len()).map(|i| (i, 0)).collect();
    for neighbors in cond_adj.values() {
        for &v in neighbors {
            *cond_in_degree.get_mut(&v).unwrap() += 1;
        }
    }

    // Collect which tables have any FK at all (as from_table)
    let tables_with_fk: HashSet<&str> = fk_edges.iter().map(|(from, _, _)| from.as_str()).collect();

    // Start with SCCs that have no incoming edges (dependency roots). Sort: (0) truly
    // independent (no FKs in or out), (1) referenced roots (others depend on them), (2) rest.
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
            (false, false) => 0, // No FKs at all → absolute top
            (_, true) => 1,      // Has dependencies to emit → second
            _ => 2,              // Has FKs but no outgoing in condensation
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

/// Append sslmode=disable to the URL if no sslmode is set, so local Postgres without TLS works.
fn ensure_sslmode_disable(url: &str) -> String {
    if url.contains("sslmode=") {
        url.to_string()
    } else {
        let sep = if url.contains('?') { "&" } else { "?" };
        format!("{}{}sslmode=disable", url, sep)
    }
}

fn format_rust_output(
    tables: &[String],
    circular_fk_cols: &HashMap<String, Vec<String>>,
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
    out
}

fn escape_rust_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
