import * as process from 'node:process';

const { Pool } = require('pg');
const { Transform } = require('stream');
const copyFrom = require('pg-copy-streams').from;

const { DB_HOST, DB_NAME, DB_USER, DB_PASSWORD, DB_PORT } = process.env;

const pool = new Pool({
  user: DB_USER,
  host: DB_HOST,
  database: DB_NAME,
  password: DB_PASSWORD,
  port: DB_PORT,
});

// Helper function to create a Transform stream for converting JSON to CSV
function jsonToCsvTransform(columnOrder) {
  let isFirstChunk = true;

  return new Transform({
    writableObjectMode: true,
    readableObjectMode: false,
    transform(record, _encoding, callback) {
      if (isFirstChunk) {
        // Push the header row (columns) based on the specified column order
        this.push(columnOrder.join(',') + '\n');
        isFirstChunk = false;
      }
      // Map the values in the record to the column order
      const csvRow = columnOrder
        .map((key) => {
          const value = record[key];

          // PostgreSQL null representation
          if (value === null) return 'null';

          // Undefined values are treated as empty fields
          if (value === undefined) return null;

          // Convert timestamps to ISO strings if the key contains "timestamp"
          if (
            key.toLowerCase().includes('timestamp') &&
            value instanceof Date
          ) {
            return `"${value.toISOString()}"`;
          }
          if (
            key.toLowerCase().includes('timestamp') &&
            typeof value === 'string'
          ) {
            try {
              return `"${new Date(value).toISOString()}"`;
            } catch {
              return value; // Keep as-is if it's not a valid date
            }
          }

          // Handle strings with escaping for CSV format
          if (typeof value === 'string') {
            return `"${value.replace(/"/g, '""')}"`;
          }

          // For all other types (number, boolean, etc.), keep as-is
          return value;
        })
        .join(',');
      this.push(csvRow + '\n');
      callback();
    },
  });
}

async function parseJsonToCsv(
  jsonData: any[],
  table_columns: string[],
): Promise<string> {
  return new Promise((resolve, reject) => {
    const chunks: string[] = [];
    const transformer = jsonToCsvTransform(table_columns);

    transformer.on('data', (chunk) => chunks.push(chunk.toString()));
    transformer.on('end', () => resolve(chunks.join('')));
    transformer.on('error', reject);

    // Simulate a JSON stream
    const jsonStream = new Transform({
      objectMode: true,
      transform(chunk, _encoding, done) {
        this.push(chunk);
        done();
      },
    });

    jsonData.forEach((obj) => jsonStream.push(obj));
    jsonStream.end();

    jsonStream.pipe(transformer);
  });
}

// Function to copy data from a JSON array to a PostgreSQL table
export async function copyData(
  tableName: string,
  jsonData: any[],
  table_columns: string[],
) {
  if (!jsonData || jsonData.length === 0) {
    throw new Error('No data to copy.');
  }

  const client = await pool.connect();
  try {
    await client.query('BEGIN');

    // Parse the data once
    const csvData = await parseJsonToCsv(jsonData, table_columns);
    await Promise.all([
      copyParsedDataToTable(client, tableName, csvData, table_columns),
      // copyParsedDataToTable(
      //   client,
      //   `temp_${tableName}`,
      //   csvData,
      //   table_columns,
      // ),
    ]);

    await client.query('COMMIT');
  } catch (error) {
    console.error('Error during COPY operation:', error);
    await client.query('ROLLBACK');
    throw error;
  } finally {
    client.release();
  }
}
async function copyParsedDataToTable(
  client: any,
  tableName: string,
  csvData: string,
  table_columns: string[],
) {
  try {
    // const copyQuery = `COPY ${tableName} (${table_columns.join(
    //   ',',
    // )}) FROM STDIN CSV HEADER`;
    const copyQuery = `COPY ${tableName} (${table_columns
      .map((col) => `"${col}"`)
      .join(',')}) FROM STDIN CSV HEADER`;
    const stream = client.query(copyFrom(copyQuery));

    stream.write(csvData);
    stream.end();

    await new Promise((resolve, reject) => {
      stream.on('finish', resolve);
      stream.on('error', reject);
    });
  } catch (error) {
    console.error(`Error during COPY operation to table ${tableName}:`, error);
    throw error;
  }
}
