const { Pool } = require('pg');
const pool = new Pool({
  user: 'admin',
  host: 'localhost',
  database: 'nullnet',
  password: 'admin',
  port: 5432,
});

export async function insertRecords(mainTableName, tempTableName, records) {
  const client = await pool.connect();
  try {
    await client.query('BEGIN'); // Start the transaction
    const transactionStartTime = Date.now();

    // Extract column names from the first record (assuming all records have the same structure)
    const columns = Object.keys(records[0]).join(', ');
    const placeholders = records
      .map(
        (_, index) =>
          `(${Object.keys(records[0])
            .map(
              (_, idx) =>
                `$${index * Object.keys(records[0]).length + idx + 1}`,
            )
            .join(', ')})`,
      )
      .join(', ');

    // Prepare SQL statements for the main and temporary tables
    const insertMainTableSql = `
      INSERT INTO ${mainTableName} (${columns})
      VALUES ${placeholders}
      RETURNING *;
    `;

    const insertTempTableSql = `
      INSERT INTO ${tempTableName} (${columns})
      VALUES ${placeholders};
    `;

    // Flatten all record values for parameterization
    const recordValues = records.flatMap((record) => Object.values(record));

    // Execute both inserts in parallel
    const resultsMainTable = await client.query(
      insertMainTableSql,
      recordValues,
    );
    await client.query(insertTempTableSql, recordValues);

    await client.query('COMMIT'); // Commit the transaction
    console.log(
      `Transaction execution: ${Date.now() - transactionStartTime}ms`,
    );

    return resultsMainTable.rows;
  } catch (error) {
    console.log(error);
    await client.query('ROLLBACK'); // Rollback the transaction on error
    throw error;
  } finally {
    client.release();
  }
}
